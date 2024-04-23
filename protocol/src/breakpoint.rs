//! Definition of Breakpoint
use crate::source::LineColumn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Breakpoint
pub struct Breakpoint {
    pub id: u32,
    pub file_id: u32,
    pub loc: LineColumn,
    pub loc_end: Option<LineColumn>,
    pub breakpoint_type: BreakpointType,
    pub capture: VariableCapture,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Reason to set this breakpoint
pub enum BreakpointType {
    #[default]
    Breakpoint,
    FunctionCall {
        /// Breaking at a specifc line of code is fuzzy, sometimes we might end up in a different location due to inlining etc.
        /// We need the function name to double check.
        fn_name: String,
    },
    FunctionReturn,
    FutureEndpoint,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Option for capturing variables.
pub enum VariableCapture {
    /// Capture all arguments
    Arguments,
    /// Capture all local variables
    Locals,
    /// Capture only these variables by name
    Only(Vec<String>),
    #[default]
    /// Capture nothing
    None,
}

impl BreakpointType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Breakpoint => "Breakpoint",
            Self::FunctionCall { .. } => "FunctionCall",
            Self::FunctionReturn => "FunctionReturn",
            Self::FutureEndpoint => "FutureEndpoint",
        }
    }
}
