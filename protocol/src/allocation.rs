use crate::util::impl_serde_with_str;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Allocation
pub struct Allocation {
    // TODO
    // thread_id
    // frame_id
    pub action: AllocAction,
    pub address: u64,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Allocation; but with borrowed str
pub struct AllocationBorrowed<'a> {
    pub action: AllocAction,
    pub address: u64,
    pub type_name: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
/// Allocation action
pub enum AllocAction {
    /// Allocation: originates from `exchange_malloc`
    Alloc,
    /// Deallocation: originates from `drop_in_place`
    Drop,
}

impl_serde_with_str!(AllocAction);
