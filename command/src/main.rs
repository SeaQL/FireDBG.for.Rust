use anyhow::{Context, Result};
use firedbg_cli::{cfg, console};
use firedbg_rust_parser::{
    parse_file, parse_workspace,
    serde::{to_bson_file, to_json_file},
    Binary, Example, File, Package, Test, Workspace,
};
use glob::glob;
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::exit,
    time::SystemTime,
};
use structopt::StructOpt;
use tokio::fs::{create_dir_all, remove_dir_all, remove_file};

const TEMPLATE: &str = concat!(
    "{bin} {version}\n",
    "  by SeaQL.org
{about}

USAGE:
    {usage}

{all-args}

AUTHORS:
    {author}
"
);

const ABOUT: &str = r#"
 _____ _          ____  ____   ____     
|  ___(_)_ __ ___|  _ \| __ ) / ___|    
| |_  | | '__/ _ \ | | |  _ \| |  _     
|  _| | | | |  __/ |_| | |_) | |_| |    
|_|   |_|_|  \___|____/|____/ \____|    
                                        
Time Travel Visual Debugger for Rust
====================================
"#;

#[derive(StructOpt, Debug)]
#[structopt(
    template = TEMPLATE,
    about = ABOUT,
    author,
)]
struct Command {
    /// Absolute path to the workspace
    #[structopt(long, global = true, default_value = "./")]
    workspace_root: String,
    #[structopt(long, global = true, env = "FIREDBG_HOME")]
    firedbg_home: Option<String>,
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Parse all `.rs` source files in the current workspace
    Cache {},
    /// Cleanup `/firedbg` folder
    Clean {},
    /// Run a binary target with debugging enabled
    Run {
        binary_name: Option<String>,
        args: Vec<String>,
        #[structopt(long)]
        output: Option<String>,
    },
    /// Run an integrated test with debugging enabled
    Test {
        test_name: String,
        testcase: String,
        args: Vec<String>,
        #[structopt(long)]
        output: Option<String>,
    },
    /// Run an unit test with debugging enabled
    UnitTest {
        package_name: String,
        testcase: String,
        args: Vec<String>,
        #[structopt(long)]
        output: Option<String>,
    },
    /// Run an example with debugging enabled
    Example {
        example_name: String,
        args: Vec<String>,
        #[structopt(long)]
        output: Option<String>,
    },
    /// List all `firedbg` runs
    ListRun {
        #[structopt(long)]
        json_format: bool,
    },
    /// List all runnable targets
    ListTarget {
        #[structopt(long)]
        json_format: bool,
    },
    /// Open debugger view in VS Code
    Open {
        #[structopt(default_value = "1")]
        idx: usize,
    },
    /// Run indexer on the latest run and save it as a `.sqlite` db file
    Index {
        #[structopt(default_value = "1")]
        idx: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Command {
        workspace_root,
        firedbg_home,
        sub_command,
    } = Command::from_args();

    let workspace = &parse_workspace(&workspace_root)
        .with_context(|| format!("Fail to parse metadata of workspace `{}`", workspace_root))?;

    log::info!("cargo_workspace\n{:#?}", workspace);

    check_firedbg_version_updated(workspace)
        .await
        .context("Fail to check FireDBG version updated")?;

    match sub_command {
        SubCommand::Cache {} => {
            cache_workspace(workspace)
                .await
                .context("Fail to cache workspace")?;
        }
        SubCommand::Clean {} => {
            remove_firedbg_dir(workspace)
                .await
                .context("Fail to remove FireDBG directory")?;
        }
        SubCommand::Run {
            binary_name,
            args,
            output,
        } => {
            let binary_names = workspace.binary_names();
            if binary_names.is_empty() {
                println!("No binary.");
                exit(1);
            }
            let list = || {
                println!("Available binaries are:");
                println!("\t{}", binary_names.join("\n\t"));
                exit(1);
            };
            let find_binary = |binary_name: &str| {
                let Some((package, binary)) = workspace.find_binary(binary_name) else {
                    println!("Unknown binary `{binary_name}`.");
                    list()
                };
                (package, binary)
            };
            let (package, binary) = if let Some(binary_name) = binary_name {
                find_binary(&binary_name)
            } else if binary_names.len() == 1 {
                find_binary(&binary_names[0])
            } else {
                list()
            };
            let trace_cfg =
                &parse_trace_config(workspace, package).context("Fail to parse trace config")?;
            cache_workspace(workspace)
                .await
                .context("Fail to cache workspace")?;
            run_binary(
                workspace,
                trace_cfg,
                package,
                binary,
                args,
                output,
                firedbg_home,
            )
            .await
            .context("Fail to debug binary")?;
        }
        SubCommand::Test {
            test_name,
            testcase,
            args,
            output,
        } => {
            // Show all available tests if the input test is unknown
            let Some((package, test)) = workspace.find_test(&test_name) else {
                println!("Unknown test `{test_name}`.");
                println!("Available tests are:");
                println!("\t{}", workspace.test_names().join("\n\t"));
                exit(1);
            };
            let testcases = test
                .get_testcases(package)
                .context("Fail to get integration test cases")?;
            if !testcases.contains(&testcase) {
                println!("Unknown test method `{test_name}`.");
                println!("Available test methods are:");
                println!("\t{}", testcases.join("\n\t"));
                exit(1);
            }
            let trace_cfg =
                &parse_trace_config(workspace, package).context("Fail to parse trace config")?;
            cache_workspace(workspace)
                .await
                .context("Fail to cache workspace")?;
            run_test(
                workspace,
                trace_cfg,
                package,
                test,
                &testcase,
                args,
                output,
                firedbg_home,
            )
            .await
            .context("Fail to debug integration test")?;
        }
        SubCommand::UnitTest {
            package_name,
            testcase,
            args,
            output,
        } => {
            let Some(package) = workspace.find_package(&package_name) else {
                let package_names = workspace.package_names();
                println!("Unknown package `{package_name}`.");
                println!("Available packages are:");
                println!("\t{}", package_names.join("\n\t"));
                exit(1);
            };
            if !package.has_lib {
                println!("Package `{package_name}` don't have a library.");
                exit(1);
            }
            let testcases = package
                .get_unit_test_names()
                .context("Fail to get unit test cases")?;
            if !testcases.contains(&testcase) {
                println!("Unknown unit test method `{testcase}`.");
                println!("Available unit test methods are:");
                println!("\t{}", testcases.join("\n\t"));
                exit(1);
            }
            let trace_cfg =
                &parse_trace_config(workspace, package).context("Fail to parse trace config")?;
            cache_workspace(workspace)
                .await
                .context("Fail to cache workspace")?;
            run_unit_test(
                workspace,
                trace_cfg,
                package,
                &testcase,
                args,
                output,
                firedbg_home,
            )
            .await
            .context("Fail to debug unit test")?;
        }
        SubCommand::Example {
            example_name,
            args,
            output,
        } => {
            // Show all available examples if the input example is unknown
            let Some((package, example)) = workspace.find_example(&example_name) else {
                println!("Unknown example `{example_name}`.");
                println!("Available examples are:");
                println!("\t{}", workspace.example_names().join("\n\t"));
                exit(1);
            };
            let trace_cfg =
                &parse_trace_config(workspace, package).context("Fail to parse trace config")?;
            cache_workspace(workspace)
                .await
                .context("Fail to cache workspace")?;
            run_example(
                workspace,
                trace_cfg,
                package,
                example,
                args,
                output,
                firedbg_home,
            )
            .await
            .context("Fail to debug example")?;
        }
        SubCommand::ListRun { json_format } => {
            let firedbg_runs = get_firedbg_runs(workspace)?;
            if !json_format {
                list_firedbg_runs(firedbg_runs);
            } else {
                let arr: Vec<_> = firedbg_runs
                    .iter()
                    .map(|firedbg_run| path_to_str(firedbg_run))
                    .collect();
                println!("{}", serde_json::json!(arr));
            }
        }
        SubCommand::ListTarget { json_format } => list_target(workspace, json_format)
            .await
            .context("Fail to list target")?,
        SubCommand::Open { idx } => {
            let firedbg_runs = get_firedbg_runs(workspace).context("Fail to get FireDBG run")?;
            let Some(firedbg_run) = firedbg_runs.get(idx - 1) else {
                println!("Unknown idx `{idx}`.");
                list_firedbg_runs(firedbg_runs);
                exit(1);
            };
            std::process::Command::new("code")
                .arg(path_to_str(firedbg_run))
                .spawn()?
                .wait()?;
        }
        SubCommand::Index { idx } => {
            let firedbg_runs = get_firedbg_runs(workspace).context("Fail to get FireDBG run")?;
            let Some(firedbg_run) = firedbg_runs.get(idx - 1) else {
                println!("Unknown idx `{idx}`.");
                list_firedbg_runs(firedbg_runs);
                exit(1);
            };
            let input = path_to_str(firedbg_run);
            let output = input.replace(".firedbg.ss", ".sqlite");
            let mut command = if env::var("CARGO_PKG_NAME").is_ok() {
                let mut command = std::process::Command::new("cargo");
                command
                    .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../indexer"))
                    .arg("run")
                    .arg("--");
                command
            } else {
                let home = firedbg_home.unwrap_or(cargo_bin()?);
                let home = home.trim_end_matches('/');
                std::process::Command::new(format!("{home}/firedbg-indexer"))
            };
            command
                .arg("--input")
                .arg(input)
                .arg("--output")
                .arg(output);

            log::info!("indexer_command\n{:?}", command);

            console::status("Running", &format!("`{:?}`", command));

            command.spawn()?.wait()?;
        }
    }

    Ok(())
}

fn parse_firedbg_config(workspace: &Workspace) -> Result<cfg::Config> {
    let workspace_root_dir = &workspace.root_dir;
    let config_file_path = &format!("{workspace_root_dir}/firedbg.toml");
    let config_file_path = Path::new(config_file_path);
    let config = if config_file_path.exists() && config_file_path.is_file() {
        let config_file_content = fs::read_to_string(&config_file_path).context("Fail to read")?;
        toml::from_str(&config_file_content).context("Fail to parse TOML")?
    } else {
        console::warn(
            "Tracing",
            "`firedbg.toml` config file not found, default settings will be applied",
        );
        cfg::Config::default()
    };
    Ok(config)
}

fn parse_trace_config<'a>(
    workspace: &'a Workspace,
    executable_package: &'a Package,
) -> Result<Vec<(&'a Package, cfg::Trace)>> {
    let config = &parse_firedbg_config(workspace).context("Fail to parse `firedbg.toml`")?;

    log::info!("firedbg_config\n{:#?}", config);

    // All package is set to trace none by default; except the executable package set to trace full
    let mut trace_packages = workspace
        .packages
        .iter()
        .fold(BTreeMap::new(), |mut acc, package| {
            let trace = if package.name == executable_package.name {
                cfg::Trace::Full
            } else {
                cfg::Trace::None
            };
            acc.insert(package.name.as_str(), (package, trace));
            acc
        });
    // Parse trace config
    for (package_name, member) in config.workspace.members.iter() {
        if let Some(package) = workspace.find_package(package_name) {
            trace_packages.insert(package.name.as_str(), (package, member.trace));
        }
    }
    let values: Vec<_> = trace_packages.into_values().collect();

    log::info!("trace_packages\n{:#?}", values);

    for (package, trace) in values.iter() {
        let package_name = package.name.as_str();
        let trace_str = trace.to_str();
        console::status(
            "Tracing",
            &format!("{package_name} = {{ trace = \"{trace_str}\" }}"),
        );
    }

    Ok(values)
}

async fn cache_workspace(workspace: &Workspace) -> Result<()> {
    let workspace_root_dir = &workspace.root_dir;
    let firedbg_dir = &workspace.get_firedbg_dir();

    let mut cache_skip = 0;
    let mut cache_refresh = 0;
    for package in workspace.packages.iter() {
        let regex = &format!("{}/**/*.rs", package.root_dir).replace("//", "/");
        let context = || format!("Invalid glob regex: `{regex}`");
        for src_file in glob(regex).with_context(context)?.filter_map(Result::ok) {
            // abs_file_path      = /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/codelldb/adapter/crates/lldb/src/sb/sbtypeenummember.rs
            // parent_dir         = /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/codelldb/adapter/crates/lldb/src/sb
            // workspace_root_dir = /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal
            // rel_path           =                                                     codelldb/adapter/crates/lldb/src/sb
            // file_name          =                                                                                         sbtypeenummember.rs
            // map_dir            = /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/firedbg/codelldb/adapter/crates/lldb/src/sb
            // map_path           = /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/firedbg/codelldb/adapter/crates/lldb/src/sb/sbtypeenummember.rs.firedbg.map
            let abs_file_path = path_to_str(&src_file);
            let parent_dir = path_to_str(src_file.parent().expect("Parent directory not found"));
            let rel_path = &parent_dir[workspace_root_dir.len()..];
            let file_name = path_file_name(&src_file);
            let map_dir = &format!("{firedbg_dir}{rel_path}");
            let map_path = &format!("{map_dir}/{file_name}.firedbg.map");

            // Skip caching if the `src_file` has older modified timestamp than the `map_file`
            let src_modified = src_file.metadata()?.modified()?;
            let map_file = Path::new(map_path);
            if map_file.is_file() {
                let map_modified = map_file.metadata()?.modified()?;
                if src_modified <= map_modified {
                    log::debug!("parser_cache_skip `{abs_file_path}`");
                    cache_skip += 1;
                    continue;
                }
            }
            log::debug!("parser_cache_create `{abs_file_path}`");
            cache_refresh += 1;

            let functions = parse_file(abs_file_path).unwrap_or_else(|e| {
                console::warn(
                    "Parsing",
                    &format!("fail to parse and cache source file `{abs_file_path}`; {e}"),
                );
                Vec::new()
            });

            let content = File {
                path: abs_file_path.into(),
                functions,
                crate_name: package.get_crate_name(),
                modified: src_modified,
            };
            create_dir_all(map_dir)
                .await
                .with_context(|| format!("Fail to create directory: `{map_dir}`"))?;
            to_bson_file(map_path, &content)
                .await
                .with_context(|| format!("Fail to create BSON file: `{map_path}`"))?;
            if log::log_enabled!(log::Level::Debug) {
                to_json_file(&format!("{map_path}.json"), &content)
                    .await
                    .with_context(|| format!("Fail to create JSON file: `{map_path}.json`"))?;
            }
        }
    }
    let cached = cache_skip + cache_refresh;
    console::status(
        "Parsed",
        &format!("{cached} source files; re-cached {cache_refresh} source files"),
    );

    Ok(())
}

fn current_firedbg_version() -> cfg::Version {
    cfg::Version {
        firedbg_cli: env!("CARGO_PKG_VERSION").into(),
    }
}

fn workspace_firedbg_version(workspace: &Workspace) -> Result<Option<cfg::Version>> {
    let version_path = &workspace.get_version_path();
    let version_path = Path::new(version_path);
    let version = if version_path.exists() && version_path.is_file() {
        let file_content = fs::read_to_string(&version_path).context("Fail to read")?;
        Some(toml::from_str(&file_content).context("Fail to parse TOML")?)
    } else {
        None
    };
    Ok(version)
}

async fn pin_firedbg_version(workspace: &Workspace) -> Result<()> {
    let version = &current_firedbg_version();
    let version_path = &workspace.get_version_path();
    let version_path = Path::new(version_path);
    let firedbg_dir = &workspace.get_firedbg_dir();

    create_dir_all(firedbg_dir)
        .await
        .with_context(|| format!("Fail to create directory: `{firedbg_dir}`"))?;
    let file_content = toml::to_string_pretty(version).context("Fail to serialize TOML")?;
    fs::write(version_path, file_content).context("Fail to write")?;
    Ok(())
}

async fn remove_firedbg_dir(workspace: &Workspace) -> Result<()> {
    let firedbg_directory = &workspace.get_firedbg_dir();
    let firedbg_dir = Path::new(firedbg_directory);
    if firedbg_dir.exists() {
        remove_dir_all(firedbg_dir)
            .await
            .with_context(|| format!("Fail to delete directory: `{firedbg_directory}`"))?;
        console::status("Cleaning", "FireDBG resources");
    }
    Ok(())
}

async fn re_create_firedbg_folder(workspace: &Workspace) -> Result<()> {
    let firedbg_directory = &workspace.get_firedbg_dir();
    let paths: Vec<_> = glob(&format!("{}/*", firedbg_directory))?
        .filter_map(Result::ok)
        .collect();
    for path in paths {
        if path.is_dir() && path.ends_with("firedbg/target") {
            // Keep the FireDBG target directory
            continue;
        } else if path.is_dir() {
            remove_dir_all(&path)
                .await
                .with_context(|| format!("Fail to delete directory: `{}`", path.display()))?;
        } else if path.is_file() {
            remove_file(&path)
                .await
                .with_context(|| format!("Fail to delete file: `{}`", path.display()))?;
        }
    }
    pin_firedbg_version(workspace)
        .await
        .context("Fail to pin FireDBG version")?;
    Ok(())
}

async fn check_firedbg_version_updated(workspace: &Workspace) -> Result<()> {
    let current_version = current_firedbg_version();
    let workspace_version = workspace_firedbg_version(workspace)?;
    match workspace_version {
        Some(workspace_version) if workspace_version != current_version => {
            console::status("Upgrading", "FireDBG version");
            re_create_firedbg_folder(workspace).await?;
        }
        None => {
            re_create_firedbg_folder(workspace).await?;
        }
        _ => (),
    }
    Ok(())
}

async fn run_binary(
    workspace: &Workspace,
    trace_cfg: &[(&Package, cfg::Trace)],
    package: &Package,
    binary: &Binary,
    args: Vec<String>,
    output: Option<String>,
    firedbg_home: Option<String>,
) -> Result<()> {
    let sub_command = "run";
    let build_cmd_output = binary.build(package).context("Fail to build binary")?;
    if !build_cmd_output.status.success() {
        panic!("Fail to compile binary");
    }
    let executable = binary.get_binary_path(workspace);
    let name = &binary.name;
    run_debugger(
        workspace,
        trace_cfg,
        sub_command,
        executable,
        name,
        &package.name,
        None,
        args,
        output,
        firedbg_home,
    )
    .await
}

async fn run_test(
    workspace: &Workspace,
    trace_cfg: &[(&Package, cfg::Trace)],
    package: &Package,
    test: &Test,
    testcase: &str,
    args: Vec<String>,
    output: Option<String>,
    firedbg_home: Option<String>,
) -> Result<()> {
    let sub_command = "test";
    let build_cmd_output = test
        .build(package)
        .context("Fail to build integration test")?;
    if !build_cmd_output.status.success() {
        panic!("Fail to compile integration test");
    }
    let executable = test
        .get_test_path(workspace, package)
        .context("Fail to get integration test executable")?;
    let name = &format!("{}-{}", test.name, testcase.replace("::", "_"));
    run_debugger(
        workspace,
        trace_cfg,
        sub_command,
        executable,
        name,
        &package.name,
        Some(testcase),
        args,
        output,
        firedbg_home,
    )
    .await
}

async fn run_unit_test(
    workspace: &Workspace,
    trace_cfg: &[(&Package, cfg::Trace)],
    package: &Package,
    testcase: &str,
    args: Vec<String>,
    output: Option<String>,
    firedbg_home: Option<String>,
) -> Result<()> {
    let sub_command = "unit-test";
    let build_cmd_output = package
        .build_unit_test()
        .context("Fail to build unit test")?;
    if !build_cmd_output.status.success() {
        panic!("Fail to compile unit test");
    }
    let executable = package
        .get_unit_test_path(workspace)
        .context("Fail to get unit test executable")?;
    let name = &format!("{}-{}", package.name, testcase.replace("::", "_"));
    run_debugger(
        workspace,
        trace_cfg,
        sub_command,
        executable,
        name,
        &package.name,
        Some(testcase),
        args,
        output,
        firedbg_home,
    )
    .await
}

async fn run_example(
    workspace: &Workspace,
    trace_cfg: &[(&Package, cfg::Trace)],
    package: &Package,
    example: &Example,
    args: Vec<String>,
    output: Option<String>,
    firedbg_home: Option<String>,
) -> Result<()> {
    let sub_command = "example";
    let build_cmd_output = example.build(package).context("Fail to build example")?;
    if !build_cmd_output.status.success() {
        panic!("Fail to compile example");
    }
    let executable = example.get_example_path(workspace);
    let name = &example.name;
    run_debugger(
        workspace,
        trace_cfg,
        sub_command,
        executable,
        name,
        &package.name,
        None,
        args,
        output,
        firedbg_home,
    )
    .await
}

async fn run_debugger(
    workspace: &Workspace,
    trace_cfg: &[(&Package, cfg::Trace)],
    sub_command: &str,
    executable: String,
    name: &str,
    package_name: &str,
    testcase: Option<&str>,
    args: Vec<String>,
    output: Option<String>,
    firedbg_home: Option<String>,
) -> Result<()> {
    let workspace_root_dir = &workspace.root_dir;
    let workspace_output_dir = workspace.get_firedbg_target_dir();
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_millis();
    let output = output.unwrap_or(format!(
        "{workspace_output_dir}/{name}-{timestamp}.firedbg.ss"
    ));

    let mut command = if env::var("CARGO_PKG_NAME").is_ok() {
        let mut command = std::process::Command::new("cargo");
        command
            .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../debugger"))
            .arg("run")
            .arg("--");
        command
    } else {
        let home = firedbg_home.unwrap_or(cargo_bin()?);
        let home = home.trim_end_matches('/');
        let mut command = std::process::Command::new(format!("{home}/firedbg-debugger"));
        let lib_path = format!("{home}/firedbg-lib/lib");
        if cfg!(target_os = "linux") {
            command.env("LD_LIBRARY_PATH", lib_path);
        } else if cfg!(target_os = "macos") {
            command.env("DYLD_FALLBACK_LIBRARY_PATH", lib_path);
        }
        command
    };
    command.arg(sub_command).arg(executable);

    if let Some(testcase) = testcase {
        command.arg(testcase);
    }

    command
        .arg("--workspace-root")
        .arg(workspace_root_dir)
        .arg("--output")
        .arg(output)
        .arg("--package-name")
        .arg(package_name);

    for (package, trace) in trace_cfg {
        let package_path = if &package.root_dir != workspace_root_dir {
            &package.root_dir[(workspace_root_dir.len() + 1)..]
        } else {
            "."
        };
        let trace = trace.to_str();
        command
            .arg("--package")
            .arg(format!("{package_path}/{trace}"));
    }

    if !args.is_empty() {
        command.arg("--").args(args);
    }

    log::info!("debugger_command\n{:?}", command);

    console::status("Running", &format!("`{:?}`", command));

    command.spawn()?.wait()?;

    Ok(())
}

fn get_firedbg_runs(workspace: &Workspace) -> Result<Vec<PathBuf>> {
    let regex = &format!("{}/*.firedbg.ss", workspace.get_firedbg_target_dir()).replace("//", "/");
    let mut target_files: Vec<_> = glob(regex)?.filter_map(Result::ok).collect();
    target_files.sort_by(|l, r| {
        let l_created = path_created(l);
        let r_created = path_created(r);
        r_created.cmp(&l_created)
    });
    Ok(target_files)
}

fn list_firedbg_runs(firedbg_runs: Vec<PathBuf>) {
    println!("Available `firedbg` runs are:");
    for (i, firedbg_run) in firedbg_runs.into_iter().enumerate() {
        let idx = i + 1;
        let file_name = path_file_name(&firedbg_run);
        println!("{idx: >5}) {file_name}");
    }
}

async fn list_target(workspace: &Workspace, json_format: bool) -> Result<()> {
    #[derive(Debug, Serialize)]
    struct Target<'a> {
        binaries: Vec<&'a Binary>,
        examples: Vec<&'a Example>,
        integration_tests: Vec<IntegrationTest<'a>>,
        unit_tests: Vec<UnitTest<'a>>,
    }

    #[derive(Debug, Serialize)]
    struct IntegrationTest<'a> {
        package_name: &'a str,
        test: &'a Test,
        test_cases: Vec<String>,
    }

    #[derive(Debug, Serialize)]
    struct UnitTest<'a> {
        package_name: &'a str,
        test_cases: Vec<String>,
    }

    let binaries: Vec<_> = workspace
        .packages
        .iter()
        .flat_map(|package| &package.binaries)
        .collect();

    let examples: Vec<_> = workspace
        .packages
        .iter()
        .flat_map(|package| &package.examples)
        .collect();

    fn handle_tests<'a>((package, test): &'a (&'a Package, Option<&'a Test>)) -> TestType {
        if let Some(test) = test {
            let test_cases = test
                .get_testcases(package)
                .expect("Fail to parse integration test");
            TestType::IntegrationTest(IntegrationTest {
                package_name: &package.name,
                test,
                test_cases,
            })
        } else {
            let test_cases = package
                .get_unit_test_names()
                .expect("Fail to parse unit test");
            TestType::UnitTest(UnitTest {
                package_name: &package.name,
                test_cases,
            })
        }
    }

    #[derive(Debug)]
    enum TestType<'a> {
        IntegrationTest(IntegrationTest<'a>),
        UnitTest(UnitTest<'a>),
    }

    let tests: Vec<_> = workspace
        .packages
        .iter()
        .flat_map(|package| {
            package
                .tests
                .iter()
                .map(|test| (package, Some(test)))
                .collect::<Vec<_>>()
        })
        .chain(
            workspace
                .packages
                .iter()
                .filter(|package| package.has_lib)
                .map(|package| (package, None)),
        )
        .collect();

    let res: Vec<_> = tests.par_iter().map(handle_tests).collect();

    let mut integration_tests = Vec::new();
    let mut unit_tests = Vec::new();
    for d in res {
        match d {
            TestType::IntegrationTest(d) => {
                if !d.test_cases.is_empty() {
                    integration_tests.push(d);
                }
            }
            TestType::UnitTest(d) => {
                if !d.test_cases.is_empty() {
                    unit_tests.push(d);
                }
            }
        }
    }

    let target = Target {
        binaries,
        examples,
        integration_tests,
        unit_tests,
    };

    if json_format {
        println!("{}", serde_json::json!(target).to_string());
    } else {
        if !target.binaries.is_empty() {
            println!("\nAvailable binaries are:");
            for binary in target.binaries.iter() {
                println!("\t{}", binary.name);
            }
        }

        if !target.examples.is_empty() {
            println!("\nAvailable examples are:");
            for example in target.examples.iter() {
                println!("\t{}", example.name);
            }
        }

        for integration_test in target.integration_tests.iter() {
            println!("\nAvailable tests of `{}` are:", integration_test.test.name);
            println!("\t{}", integration_test.test_cases.join("\n\t"));
        }

        for unit_test in target.unit_tests.iter() {
            println!(
                "\nAvailable unit tests of `{}` package are:",
                unit_test.package_name
            );
            println!("\t{}", unit_test.test_cases.join("\n\t"));
        }
    }

    let firedbg_dir = &workspace.get_firedbg_dir();
    create_dir_all(firedbg_dir)
        .await
        .with_context(|| format!("Fail to create directory: `{firedbg_dir}`"))?;
    let path = &format!("{firedbg_dir}/target.json");
    to_json_file(path, &target)
        .await
        .with_context(|| format!("Fail to create JSON file: `{path}`"))?;
    Ok(())
}

fn path_file_name(path: &Path) -> &str {
    os_str_to_str(path.file_name().expect("Not a file"))
}

fn path_created(path: &Path) -> SystemTime {
    path.metadata()
        .expect("No metadata")
        .created()
        .expect("No created time")
}

fn path_to_str(path: &Path) -> &str {
    path.to_str().expect("Failed to convert Path to &str")
}

fn os_str_to_str(os_str: &OsStr) -> &str {
    os_str.to_str().expect("Failed to convert OsStr to &str")
}

fn cargo_bin() -> Result<String> {
    let res = if let Ok(cargo_home) = env::var("CARGO_HOME") {
        format!("{cargo_home}/bin")
    } else if let Ok(home) = env::var("HOME") {
        format!("{home}/.cargo/bin")
    } else {
        anyhow::bail!("Fail to parse cargo home")
    };
    Ok(res)
}
