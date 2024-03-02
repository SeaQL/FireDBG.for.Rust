//! Data structures for Debugger Info

use crate::util::impl_serde_with_str;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub const FIRE_DBG_FOR_RUST: &str = "FireDBG.for.Rust";
pub const INFO_STREAM: &str = "info";
pub const FILE_STREAM: &str = "file";
pub const BREAKPOINT_STREAM: &str = "breakpoint";
pub const EVENT_STREAM: &str = "event";
pub const ALLOCATION_STREAM: &str = "allocation";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
/// Information of the debugger run.
pub enum InfoMessage {
    Debugger(DebuggerInfo),
    Exit(ProgExitInfo),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Debugger Info
pub struct DebuggerInfo {
    /// The debugger engine
    pub debugger: FireDbgForRust,
    /// FireDBG version
    pub version: String,
    pub workspace_root: String,
    pub package_name: String,
    /// The target executable
    pub target: String,
    /// Arguments to the executable
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Program Exit Info
pub struct ProgExitInfo {
    pub exit_code: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Our magic pass phrase.
pub struct FireDbgForRust;

impl Display for FireDbgForRust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", FIRE_DBG_FOR_RUST)
    }
}

impl FromStr for FireDbgForRust {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == FIRE_DBG_FOR_RUST {
            Ok(Self)
        } else {
            Err("Invalid FireDbgForRust marker")
        }
    }
}

impl InfoMessage {
    pub fn redacted(&mut self) {
        match self {
            InfoMessage::Debugger(ref mut debugger_info) => {
                debugger_info.redacted();
            }
            InfoMessage::Exit(_) => (),
        }
    }
}

impl DebuggerInfo {
    pub fn redacted(&mut self) {
        self.version = "<redacted>".into();

        let path = std::path::Path::new(&self.workspace_root);
        let file_name = path.file_name().expect("file").to_str().expect("str");
        self.workspace_root = format!("<redacted>/{file_name}");

        let path = std::path::Path::new(&self.target);
        let file_name = path.file_name().expect("file").to_str().expect("str");
        self.target = format!("<redacted>/{file_name}");
    }
}

impl_serde_with_str!(FireDbgForRust);
