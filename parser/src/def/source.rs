use crate::FunctionDef;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub functions: Vec<FunctionDef>,
    pub crate_name: String,
    pub modified: SystemTime,
}
