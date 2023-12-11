#[cfg(feature = "debugger")]
#[derive(Debug, thiserror::Error)]
#[error("Write Value Error")]
pub struct WriteErr;

mod base;
#[cfg(feature = "debugger")]
mod value_type;
#[cfg(feature = "debugger")]
mod writer;

pub use base::*;
#[cfg(feature = "debugger")]
pub use value_type::*;
#[cfg(feature = "debugger")]
pub(crate) use writer::*;
