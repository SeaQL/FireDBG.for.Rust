pub mod raw;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::{Command, Output, Stdio};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    pub packages: Vec<Package>,
    pub target_dir: String,
    pub root_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub root_dir: String,
    pub dependencies: Vec<Dependency>,
    pub binaries: Vec<Binary>,
    pub tests: Vec<Test>,
    pub examples: Vec<Example>,
    pub has_lib: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub default_features: bool,
    pub features: Vec<String>,
    pub root_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binary {
    pub name: String,
    pub src_path: String,
    pub required_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    pub name: String,
    pub src_path: String,
    pub required_features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Example {
    pub name: String,
    pub src_path: String,
    pub required_features: Vec<String>,
}

impl Workspace {
    const FIREDBG_DIR: &'static str = "firedbg";
    const FIREDBG_TARGET_DIR: &'static str = "target";

    pub fn get_firedbg_dir(&self) -> String {
        format!("{}/{}", self.root_dir, Self::FIREDBG_DIR)
    }

    pub fn get_firedbg_target_dir(&self) -> String {
        format!(
            "{}/{}/{}",
            self.root_dir,
            Self::FIREDBG_DIR,
            Self::FIREDBG_TARGET_DIR,
        )
    }

    pub fn get_version_path(&self) -> String {
        format!("{}/{}/version.toml", self.root_dir, Self::FIREDBG_DIR)
    }

    pub fn find_binary(&self, binary_name: &str) -> Option<(&Package, &Binary)> {
        let mut res = None;
        'package_iter: for package in self.packages.iter() {
            for binary in package.binaries.iter() {
                if binary.name == binary_name {
                    res = Some((package, binary));
                    break 'package_iter;
                }
            }
        }
        res
    }

    pub fn find_test(&self, test_name: &str) -> Option<(&Package, &Test)> {
        let mut res = None;
        'package_iter: for package in self.packages.iter() {
            for test in package.tests.iter() {
                if test.name == test_name {
                    res = Some((package, test));
                    break 'package_iter;
                }
            }
        }
        res
    }

    pub fn find_example(&self, example_name: &str) -> Option<(&Package, &Example)> {
        let mut res = None;
        'package_iter: for package in self.packages.iter() {
            for example in package.examples.iter() {
                if example.name == example_name {
                    res = Some((package, example));
                    break 'package_iter;
                }
            }
        }
        res
    }

    pub fn find_package(&self, package_name: &str) -> Option<&Package> {
        self.packages
            .iter()
            .find(|&package| package.name == package_name)
    }

    pub fn package_names(&self) -> Vec<&str> {
        self.packages
            .iter()
            .map(|package| package.name.as_str())
            .collect()
    }

    pub fn binary_names(&self) -> Vec<&str> {
        self.packages
            .iter()
            .flat_map(|package| {
                package
                    .binaries
                    .iter()
                    .map(|binary| binary.name.as_str())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn test_names(&self) -> Vec<&str> {
        self.packages
            .iter()
            .flat_map(|package| {
                package
                    .tests
                    .iter()
                    .map(|test| test.name.as_str())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn example_names(&self) -> Vec<&str> {
        self.packages
            .iter()
            .flat_map(|package| {
                package
                    .examples
                    .iter()
                    .map(|example| example.name.as_str())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl Package {
    pub fn get_crate_name(&self) -> String {
        self.name.replace('-', "_")
    }

    pub fn get_firedbg_dir(&self, workspace: &Workspace) -> String {
        let package_rel_path = &self.root_dir[(workspace.root_dir.len() + 1)..];
        format!("{}/{}", workspace.get_firedbg_dir(), package_rel_path)
    }

    pub fn get_unit_test_cmd(&self) -> Command {
        let root_dir = &self.root_dir;
        let package_name = &self.name;
        let mut cmd = Command::new("cargo");
        cmd.arg("test")
            .arg("--manifest-path")
            .arg(format!("{root_dir}/Cargo.toml"))
            .arg("--lib")
            .arg("--package")
            .arg(package_name);
        cmd
    }

    //     Finished test [unoptimized + debuginfo] target(s) in 0.01s
    //      Running unittests src/lib.rs (target/debug/deps/quick_sort-c42cff5519f79ed2)
    // conquer::test::test_partition: test
    // conquer::test::test_partition2: test
    // test::test_quicksort: test
    // test::test_quicksort2: test
    //
    // 4 tests, 0 benchmarks
    pub fn get_unit_test_names(&self) -> Result<Vec<String>> {
        let output = self
            .get_unit_test_cmd()
            .arg("--color")
            .arg("always")
            .arg("--")
            .arg("--list")
            .output()?;
        if !output.status.success() {
            eprint!("{}", std::str::from_utf8(&output.stderr)?);
            std::process::exit(1);
        }
        let stdout = String::from_utf8(output.stdout)?;
        Ok(parse_test_testcases(stdout))
    }

    //     Finished test [unoptimized + debuginfo] target(s) in 0.02s
    //   Executable unittests src/lib.rs (target/debug/deps/quick_sort-c42cff5519f79ed2)
    pub fn build_unit_test(&self) -> Result<Output> {
        let output = self
            .get_unit_test_cmd()
            .arg("--no-run")
            .stderr(Stdio::inherit())
            .output()?;
        Ok(output)
    }

    /// Get path of unit test executable.
    ///
    /// # Panics
    ///
    /// Panic if it fail to parse unit test executable
    pub fn get_unit_test_path(&self, workspace: &Workspace) -> Result<String> {
        let output = self.get_unit_test_cmd().arg("--no-run").output()?;
        if !output.status.success() {
            eprint!("{}", std::str::from_utf8(&output.stderr)?);
            panic!("Fail to compile unit test");
        }

        let target_dir = &workspace.target_dir;
        let test_binary_prefix = "debug/deps/";
        let suffix = String::from_utf8(output.stderr)?
            .split_once(test_binary_prefix)
            .map(|(_, after)| after.chars().take_while(|c| ')'.ne(c)).collect::<String>())
            .expect("Fail to parse unit test executable path");
        // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/deps/quick_sort-c42cff5519f79ed2
        Ok(format!("{target_dir}/{test_binary_prefix}{suffix}"))
    }
}

impl Binary {
    // $ cargo build --manifest-path /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/Cargo.toml --bin main-one
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
    pub fn get_build_cmd(&self, package: &Package) -> Command {
        let root_dir = &package.root_dir;
        let binary_name = &self.name;
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--manifest-path")
            .arg(format!("{root_dir}/Cargo.toml"))
            .arg("--bin")
            .arg(binary_name);
        if !self.required_features.is_empty() {
            cmd.arg("--features").arg(self.required_features.join(","));
        }
        cmd
    }

    pub fn build(&self, package: &Package) -> Result<Output> {
        let output = self
            .get_build_cmd(package)
            .stderr(Stdio::inherit())
            .output()?;
        Ok(output)
    }

    // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/main-one
    pub fn get_binary_path(&self, workspace: &Workspace) -> String {
        let target_dir = &workspace.target_dir;
        let binary_name = &self.name;
        format!("{target_dir}/debug/{binary_name}")
    }
}

impl Test {
    // $ cargo test --manifest-path /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/Cargo.toml --test simple_tests --no-run
    //     Finished test [unoptimized + debuginfo] target(s) in 0.03s
    //   Executable tests/simple_tests.rs (tests/example-workspace/target/debug/deps/simple_tests-eb63f31f8208c8a4)
    pub fn build(&self, package: &Package) -> Result<Output> {
        let output = self
            .get_run_cmd(package)
            .arg("--no-run")
            .stderr(Stdio::inherit())
            .output()?;
        Ok(output)
    }

    /// Get path of integration test executable.
    ///
    /// # Panics
    ///
    /// Panic if it fail to parse integration test executable
    pub fn get_test_path(&self, workspace: &Workspace, package: &Package) -> Result<String> {
        let output = self.get_run_cmd(package).arg("--no-run").output()?;
        if !output.status.success() {
            eprint!("{}", std::str::from_utf8(&output.stderr)?);
            panic!("Fail to compile integration test");
        }

        let target_dir = &workspace.target_dir;
        let test_binary_prefix = "debug/deps/";
        let suffix = String::from_utf8(output.stderr)?
            .split_once(test_binary_prefix)
            .map(|(_, after)| after.chars().take_while(|c| ')'.ne(c)).collect::<String>())
            .expect("Fail to parse integration test executable path");
        // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/deps/simple_tests-eb63f31f8208c8a4
        Ok(format!("{target_dir}/{test_binary_prefix}{suffix}"))
    }

    pub fn get_run_cmd(&self, package: &Package) -> Command {
        let root_dir = &package.root_dir;
        let test_name = &self.name;
        let mut cmd = Command::new("cargo");
        cmd.arg("test")
            .arg("--manifest-path")
            .arg(format!("{root_dir}/Cargo.toml"))
            .arg("--test")
            .arg(test_name);
        if !self.required_features.is_empty() {
            cmd.arg("--features").arg(self.required_features.join(","));
        }
        cmd
    }

    //     Finished test [unoptimized + debuginfo] target(s) in 0.03s
    //      Running tests/simple_tests.rs (/Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/deps/simple_tests-69a7a6c625a5bad6)
    // main: test
    // main2: test
    //
    // 2 tests, 0 benchmarks
    pub fn get_testcases(&self, package: &Package) -> Result<Vec<String>> {
        let output = self
            .get_run_cmd(package)
            .arg("--color")
            .arg("always")
            .arg("--")
            .arg("--list")
            .output()?;
        if !output.status.success() {
            eprint!("{}", std::str::from_utf8(&output.stderr)?);
            std::process::exit(1);
        }
        let stdout = String::from_utf8(output.stdout)?;
        Ok(parse_test_testcases(stdout))
    }
}

impl Example {
    // $ cargo build --manifest-path /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/Cargo.toml --example demo
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.03s
    pub fn get_build_cmd(&self, package: &Package) -> Command {
        let root_dir = &package.root_dir;
        let example_name = &self.name;
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--manifest-path")
            .arg(format!("{root_dir}/Cargo.toml"))
            .arg("--example")
            .arg(example_name);
        if !self.required_features.is_empty() {
            cmd.arg("--features").arg(self.required_features.join(","));
        }
        cmd
    }

    pub fn build(&self, package: &Package) -> Result<Output> {
        let output = self
            .get_build_cmd(package)
            .stderr(Stdio::inherit())
            .output()?;
        Ok(output)
    }

    // /Applications/MAMP/htdocs/FireDBG.for.Rust.Internal/parser/tests/example-workspace/target/debug/examples/demo
    pub fn get_example_path(&self, workspace: &Workspace) -> String {
        let target_dir = &workspace.target_dir;
        let example_name = &self.name;
        format!("{target_dir}/debug/examples/{example_name}")
    }
}

fn parse_test_testcases(text: String) -> Vec<String> {
    let suffix = ": test";
    text.lines()
        .filter(|line| line.ends_with(suffix))
        .filter_map(|line| line.split_once(suffix).map(|(before, _)| before.to_owned()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_test_testcases() {
        assert_eq!(
            parse_test_testcases(
                r#"
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
    Running tests/simple_tests.rs (target/debug/deps/simple_tests-69a7a6c625a5bad6)
main: test
main2: test

2 tests, 0 benchmarks"#
                    .into()
            ),
            ["main", "main2"]
        );

        assert_eq!(
            parse_test_testcases(
                r#"
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
    Running unittests src/lib.rs (target/debug/deps/quick_sort-c42cff5519f79ed2)
conquer::test::test_partition: test
conquer::test::test_partition2: test
test::test_quicksort: test
test::test_quicksort2: test

4 tests, 0 benchmarks"#
                    .into()
            ),
            [
                "conquer::test::test_partition",
                "conquer::test::test_partition2",
                "test::test_quicksort",
                "test::test_quicksort2",
            ]
        );
    }
}
