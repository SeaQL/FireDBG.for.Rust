use anyhow::{Context, Result};
use firedbg_rust_debugger::{
    check_rustc_version, get_target_basename, new_breakpoint, Debugger, DebuggerInfo,
    DebuggerParams, FireDbgForRust, InfoMessage, SourceFile, INFO_STREAM,
};
use firedbg_rust_parser::{serde::from_bson_file, File};
use glob::glob;
use sea_streamer::{
    export::futures::{select, FutureExt},
    file::{FileId, FileSource, ReadFrom},
    runtime::spawn_task,
    Producer, SeaConnectOptions, SeaProducer, SeaStreamer, StreamKey, Streamer,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use structopt::StructOpt;
use tokio::{fs::create_dir_all, sync::Notify};

const TEMPLATE: &str = concat!(
    "{bin} {version} (rustc ",
    env!("RUSTC_VERSION"),
    ")\n",
    "  by SeaQL.org

USAGE:
    {usage}

{all-args}

AUTHORS:
    {author}
"
);

#[derive(StructOpt, Debug)]
#[structopt(
    template = TEMPLATE,
    author,
)]
struct Command {
    /// Absolute path to the workspace
    #[structopt(long, global = true, default_value = "./")]
    workspace_root: String,
    /// Output path for the `.firedbg.ss` file
    #[structopt(long, global = true, default_value = "./output.firedbg.ss")]
    output: String,
    /// Package name
    #[structopt(long, global = true, default_value = "")]
    package_name: String,
    /// Package configurations
    #[structopt(long = "package", global = true, parse(try_from_str = parse_package_cfg))]
    package_cfgs: Vec<PackageCfg>,
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Run a binary with `firedbg` debugging enabled
    Run {
        binary_executable: String,
        args: Vec<String>,
    },
    /// Run an integrated test with `firedbg` debugging enabled
    Test {
        test_executable: String,
        testcase: String,
        args: Vec<String>,
    },
    /// Run an unit test with `firedbg` debugging enabled
    UnitTest {
        test_executable: String,
        testcase: String,
        args: Vec<String>,
    },
    /// Run an example with `firedbg` debugging enabled
    Example {
        example_executable: String,
        args: Vec<String>,
    },
}

#[derive(Debug)]
struct PackageCfg {
    package: String,
    trace: String,
}

fn parse_package_cfg(src: &str) -> Result<PackageCfg> {
    let (package, trace) = src.split_once('/').expect("delimiter");
    Ok(PackageCfg {
        package: package.into(),
        trace: trace.into(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    check_rustc_version();

    let Command {
        workspace_root,
        package_cfgs,
        output,
        package_name,
        sub_command,
    } = Command::from_args();

    let workspace_root = workspace_root.trim_end_matches('/');
    let producer = create_streamer(&output)
        .await
        .context("Fail to create streamer")?;

    let mut files = vec![Default::default()];
    let mut breakpoints = vec![Default::default()];
    let mut id = 1;

    let get_map_file = |src_file: &PathBuf| {
        let src_rel_path = &src_file.to_str().expect("path to str")[(workspace_root.len() + 1)..];
        format!("{workspace_root}/firedbg/{src_rel_path}.firedbg.map")
    };

    let mut set_file_breakpoint = |file: File| {
        let File {
            path,
            functions,
            crate_name,
            modified,
        } = file;
        files.push(SourceFile {
            id,
            path,
            crate_name,
            modified,
        });
        for func in functions {
            breakpoints.push(new_breakpoint(breakpoints.len() as u32, id, func));
        }
        id += 1;
    };

    let get_arguments = |testcase: String| vec!["--exact".into(), testcase];

    let (binary, src_dirs, arguments) = match sub_command {
        SubCommand::Run {
            binary_executable,
            args,
        } => (binary_executable, ["src", "bin"], args),
        SubCommand::Test {
            test_executable,
            testcase,
            args,
        } => (
            test_executable,
            ["src", "tests"],
            [args, get_arguments(testcase)].concat(),
        ),
        SubCommand::UnitTest {
            test_executable,
            testcase,
            args,
        } => (
            test_executable,
            ["src", "tests"],
            [args, get_arguments(testcase)].concat(),
        ),
        SubCommand::Example {
            example_executable,
            args,
        } => (example_executable, ["src", "examples"], args),
    };

    for PackageCfg { package, .. } in package_cfgs
        .iter()
        .filter(|package_cfg| matches!(package_cfg.trace.as_str(), "full" | "call-only"))
    {
        for src_dir in src_dirs.iter() {
            let src_regex = &format!("{workspace_root}/{package}/{src_dir}/**/*.rs");
            log::debug!("src_regex `{}`", src_regex);
            let context = || format!("Invalid glob regex: `{src_regex}`");
            for src_file in glob(src_regex)
                .with_context(context)?
                .filter_map(Result::ok)
            {
                let map_file = get_map_file(&src_file);
                log::debug!("map_file `{}`", map_file);
                let file = from_bson_file(&map_file)
                    .await
                    .with_context(|| format!("Fail to deserialize BSON file: `{map_file}`"))?;
                set_file_breakpoint(file)
            }
        }
    }

    producer.send_to(
        &StreamKey::new(INFO_STREAM)
            .with_context(|| format!("Fail to create StreamKey: `{INFO_STREAM}`"))?,
        serde_json::to_string(&InfoMessage::Debugger(DebuggerInfo {
            debugger: FireDbgForRust,
            version: env!("CARGO_PKG_VERSION").to_owned(),
            workspace_root: workspace_root.to_owned(),
            package_name,
            target: binary.clone(),
            arguments: arguments.clone(),
        }))
        .context("Fail to serialize")?
        .as_str(),
    )?;

    let debugger_params = DebuggerParams {
        binary,
        files,
        breakpoints,
        arguments,
    };

    let notify = Arc::new(Notify::new());
    let notifier = notify.clone();

    // Tail program stdout
    let tail_handle = spawn_task::<_, Result<()>>(async move {
        // We need to create an empty file to be able to tail it,
        // stdout messages will be appended to the file
        let target_basename = get_target_basename(&output);
        let path = format!("{target_basename}.stdout");
        std::fs::File::create(&path).with_context(|| format!("Fail to create file: `{path}`"))?;
        let file_id = FileId::new(path);

        let mut source = FileSource::new(file_id.clone(), ReadFrom::Beginning)
            .await
            .context("Fail to start file source")?;

        loop {
            select! {
                res = FileSource::stream_bytes(&mut source).fuse() => {
                    print_to_stdout(res.context("read stdout")?)?;
                }
                _ = notify.notified().fuse() => {
                    break;
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let mut buffer = source.drain().await;
        if !buffer.is_empty() {
            print_to_stdout(buffer.consume(buffer.size()))?;
        }
        Ok(())
    });

    // Run the debugger
    Debugger::run(debugger_params, producer.clone());

    // Cleanup and kill the tail task
    producer.end().await.context("Fail to kill producer")?;
    notifier.notify_one();
    tail_handle.await??;

    Ok(())
}

pub async fn create_streamer(output: &str) -> Result<SeaProducer> {
    if let Some(dir) = Path::new(output).parent() {
        create_dir_all(dir)
            .await
            .with_context(|| format!("Fail to create directory: `{}`", dir.display()))?;
    }
    let file_id = FileId::new(output);
    let mut options = SeaConnectOptions::default();
    options.set_file_connect_options(|options| {
        options.set_create_only(true);
        options.set_end_with_eos(true);
    });
    let uri = file_id.to_streamer_uri().context("Fail to get URI")?;
    let streamer = SeaStreamer::connect(uri, options)
        .await
        .context("Fail to connect streamer")?;
    let producer = streamer
        .create_generic_producer(Default::default())
        .await
        .context("Fail to create producer")?;

    Ok(producer)
}

fn print_to_stdout(bytes: sea_streamer::file::Bytes) -> Result<()> {
    print!(
        "{}",
        std::str::from_utf8(&bytes.bytes()).context("read utf8")?
    );
    std::io::Write::flush(&mut std::io::stdout()).context("flush")
}
