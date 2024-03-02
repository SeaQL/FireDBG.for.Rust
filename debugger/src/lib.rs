//! ## FireDBG Debugger Engine for Rust
//!
//! Based on [lldb](https://lldb.llvm.org/).
//!
//! This library is semver exempt. The version number is intended to track rustc's version.
//!
//! ### Debugger Config
//!
//! Configuration can be set via the `FIREDBG_RUST_CONFIG` environment variable. e.g. `FIREDBG_RUST_CONFIG=MAX_ARRAY_SIZE=100;DONT_TRACE_ALLOCATION`
//!
//! | Key | Type | Description |
//! |:---:|:----:|:-----------:|
//! | `MAX_ARRAY_SIZE` | `usize` | Maximum number of items in array, string and other containers |
//! | `RECURSIVE_DEREF_LIMIT` | `usize` | Recursive limit; i.e. this limits the depth of a binary tree |
//! | `KEEP_HASH_ORDER` | `bool` | If set, don't sort hash maps by hash key |
//! | `DONT_TRACE_ALLOCATION` | `bool` | If set, don't trace heap allocations |
//!
//! ### Instruction Set
//!
//! Currently supports x64 (aka amd64) and arm64 (aka aarch64). There are quite some assembly parsing and register fiddling catered to each platform. There are some assumptions to pointers being 8 bytes in the codebase. It requires considerable effort to support a new architecture, but we are open to commercial collaboration.
//!
//! ### Operating Systems
//!
//! There is no OS specific code for now. lldb is used on both Linux and macOS. But on macOS we'd connect to the host's `lldb-server`.
//! If we decided to support Windows natively, we'd need to make a Windows backend.
//!
//! The `debugger` binary must be compiled for the same platform as the target binary.
//! In addition, we assume they both use the same `rustc` (not necessarily exactly the same, but ABI compatible with each other).
//!
//! ### Standard Library Types
//!
//! We intend to build-in all handling of standard library types. For `HashMap`, [frozen-hashbrown](https://github.com/tyt2y3/frozen-hashbrown) is used.
//! In the future, we want to open scripting interface (maybe via [rhai](https://rhai.rs/)) to handle vendor library types (e.g. `DateTime`, `Decimal`), in a similar sense to Natvis.
//!
//! ### Binary Value Format
//!
//! The format for serializing Rust values can be best understood by reading `SourceReader::read_values()` in `reader.rs`.
//! It should be pretty straight-forward, the only tricky part is `ReaderContext` which is for resolving memory references.
//!
//! ### Return Value Capture
//!
//! This is highly architecture specific. We try to capture the return value at the moment the function returns, i.e. at the `ret` instruction. Not everything is on the stack, sometimes the return value will be passed through registers.
//!
//! According to Rust's ABI convention, it means if the return value is:
//!
//! 1. One or two primitives, each no bigger than 64 bits. This includes `(i64, i64)` and `struct { a: i64, b: i64 }`.
//! 2. One `i128` / `u128`; will be split into `rax` / `rdx`
//! 3. One or two `f32` / `f64`; will be put in `xmm0` / `xmm1`
//! 4. `Option<T>` and `Result<T, E>`; where `T` & `E` is no bigger than 64 bits
//!
//!     The enum discriminant is in `rax`, where:
//!
//!     | Type | T | E |
//!     |:----:|:-:|:-:|
//!     | `Option` | `Some = 1` | `None = 0` |
//!     | `Result` | `Ok = 0` | `Err = 1` |
//!
//!     and the `T` / `E` will be in `rdx`.
//!
//! Unfortunately it is much more complicated than [the above summary](https://darkcoding.net/software/rust-multiple-return-types/). Right now the implementation is all mess, and completely based on heuristics.
//! If we got the chance to do this properly, may be we can generate rust code and inspect the assembly (systematically).
//! Say our return type is `(T, F)`:
//!
//! ```ignore
//! fn probe() -> (T, F) { todo!() }
//! fn extractor() {
//!     let res = probe();
//!     std::hint::black_box(&res);
//! }
//! ```
//!
//! If we disassemble the binary, we should see:
//!
//! ```ignore
//! extractor:
//! call probe
//! mov ..
//! call black_box
//! ```
//!
//! So between `probe` and `black_box`, there would be some memory fiddling, which would end up writing the full struct onto the stack, where the address will then be stored in `rax` / `x0`.
//!
//! There should be better ways to do this, if you have an idea please open a discussion thread!
//!
//! ### Heap allocation
//!
//! Right now it is still a WIP. We can trace all `Box`, `Rc`, `Arc` allocations now, so that we are able to extract the content of `Box<dyn T>`.
//! The TODO is to trace deallocations and output the information to a dedicated event stream.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    missing_debug_implementations,
    clippy::print_stderr,
    clippy::print_stdout
)]

mod bytes;
#[cfg(feature = "debugger")]
mod debugger;
mod event;
mod reader;
mod rvalue;
pub mod typename;
mod value;
pub mod version;

pub use bytes::*;
#[cfg(feature = "debugger")]
pub use debugger::*;
pub use event::*;
pub use reader::*;
use rvalue::*;
use value::*;

pub use firedbg_protocol::{allocation::*, breakpoint::*, event::*, info::*, source::*, value::*};
