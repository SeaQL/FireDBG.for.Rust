//! ## FireDBG Event Stream Protocol
//!
//! The FireDBG Event Stream is serialized according to the SeaStreamer File Format, which by convention has the `.ss` extension.
//! The Protocol defines the different streams and formats of the messages on top of the file format, and thus they have the `.firedbg.ss` extension.
//! The file format is not tightly-coupled with the stream protocol, as it is possible to stream to/from a different backend, e.g. Redis.
//!
//! There are currently 4 streams:
//!
//! | Stream Key | Format | Description |
//! |:----------:|:------:|:-----------:|
//! | `info` | Json | DebuggerInfo: debugger version, debug target, arguments and exit code, etc |
//! | `file` | Json | SourceFile: relative path to the source file |
//! | `breakpoint` | Json | Breakpoint: breakpoints created and the source location |
//! | `event` | Binary | Event: function call, function return, etc |
//! | `allocation` | Json | Allocation: allocations and deallocations |
pub use indexmap::IndexMap;

pub mod allocation;
pub mod breakpoint;
pub mod event;
pub mod info;
pub mod source;
mod util;
pub mod value;
