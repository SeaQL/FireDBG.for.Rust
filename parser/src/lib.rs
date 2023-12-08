//! ## FireDBG Source Parser for Rust
//!
//! Based on [`syn`](https://github.com/dtolnay/syn).
//!
//! `firedbg-rust-parser` is a Rust source code parser. It can parse a Rust source file, walk the abstract syntax tree of Rust, then produce a list of breakpoints for the debugger to pause the program at the beginning of every function call.
//!
//! ### Walking the AST
//!
//! We will walk the Rust AST, [`syn::Item`](https://docs.rs/syn/latest/syn/enum.Item.html), and collect all forms of function / method:
//!
//! 1. Free standalone function, [`syn::Item::Fn`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Fn)
//! 2. Impl function, [`syn::Item::Impl`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Impl)
//! 3. Trait default function, [`syn::Item::Trait`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Trait)
//! 4. Impl trait function, [`syn::Item::Impl`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Impl)
//! 5. Nested function, walking the [`syn::Item`](https://docs.rs/syn/latest/syn/enum.Item.html) recursively
//! 6. Function defined inside inline module, [`syn::Item::Mod`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Mod)
//!
//! ### Breakable Span
//!
//! A span is a region of source code, denoted by a ranged line and column number tuple, along with macro expansion information.
//! It allows the debugger to set a breakpoint at the correct location. The debugger will set the breakpoint either at the start or the end of the breakable span.
//!
//! ```ignore
//! fn func() -> i32 {
//! /*                ^-- Start of Breakable Span: (Line 1, Column 19)  */
//!     let mut i = 0;
//! /*  ^-- End of Breakable Span: (Line 3, Column 5)  */
//!     for _ in (1..10) {
//!         i += 1;
//!     }
//!     i
//! }
//! ```
//!
//! ### Ideas
//!
//! The current implementation is rudimentary, but we get exactly what we need. We considered embedding Rust Analyzer, for a few advantages: 1) to get the fully-qualified type names
//! 2) to traverse the static call graph. The problem is resource usage: we'd end up running the compiler frontend thrice (by cargo, by language server, by firedbg).
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    missing_debug_implementations,
    clippy::missing_panics_doc,
    clippy::unwrap_used,
    clippy::print_stderr,
    clippy::print_stdout
)]

pub mod def;
pub use def::*;

mod parsing;
use parsing::*;

pub mod serde;

use anyhow::{Context, Result};
use glob::glob;
use std::{
    io::Read,
    path::Path,
    process::{Command, Stdio},
    time::SystemTime,
};

pub fn parse_file<T>(path: T) -> Result<Vec<FunctionDef>>
where
    T: AsRef<Path>,
{
    let path = path.as_ref();
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("Fail to open file: `{}`", path.display()))?;
    let mut source_code = String::new();
    file.read_to_string(&mut source_code)
        .with_context(|| format!("Fail to read file: `{}`", path.display()))?;
    let res = syn::parse_file(&source_code)
        .with_context(|| format!("Fail to parse file: `{}`", path.display()))?
        .items
        .into_iter()
        .fold(Vec::new(), |mut acc, item| {
            acc.extend(item.parse());
            acc
        });
    Ok(res)
}

pub fn parse_directory<T>(directory: T) -> Result<Vec<File>>
where
    T: Into<String>,
{
    let regex = format!("{}/**/*.rs", directory.into()).replace("//", "/");
    let mut res = Vec::new();
    let context = || format!("Invalid glob regex: `{regex}`");
    for path in glob(&regex).with_context(context)?.filter_map(Result::ok) {
        let modified = path_created(&path);
        let file_path = path_to_str(&path).into();
        let functions = parse_file(&path)
            .with_context(|| format!("Fail to parse file: `{}`", path.display()))?;
        res.push(File {
            path: file_path,
            functions,
            crate_name: "".into(),
            modified,
        });
    }
    Ok(res)
}

pub fn parse_workspace<T>(directory: T) -> Result<Workspace>
where
    T: Into<String>,
{
    let dir = directory.into();
    let dir = dir.trim_end_matches('/');

    let res = Command::new("cargo")
        .current_dir(dir)
        .arg("metadata")
        .arg("--format-version=1")
        .stderr(Stdio::inherit())
        .output()?;

    // println!("{:#?}", res);

    if !res.status.success() {
        panic!("Fail to parse workspace metadata");
    }

    let workspace_raw =
        serde_json::from_slice(&res.stdout).context("Fail to deserialize JSON string")?;

    // println!("{:#?}", workspace_raw);

    Ok(parsing::parse_workspace(workspace_raw))
}

fn path_to_str(path: &Path) -> &str {
    path.to_str().expect("Failed to convert Path to &str")
}

fn path_created(path: &Path) -> SystemTime {
    path.metadata()
        .expect("No metadata")
        .created()
        .expect("No created time")
}
