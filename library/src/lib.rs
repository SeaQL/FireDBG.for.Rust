//! ## FireDBG Support Library
//!
//! ### `fire::dbg!`
//!
//! This macro allows you to capture the value of a variable via runtime inspection in FireDBG.
//!
//! Usage example:
//!
//! ```
//! use firedbg_lib::fire;
//!
//! fn some_fn(v: i32) -> i32 {
//!     fire::dbg!(v) + 1
//! }
//!
//! fn other_fn(v: i32) -> i32 {
//!     fire::dbg!("arg_v", v) + 1
//! }
//! ```
//!
//! Which `fire::dbg!(v)` would expand to `__firedbg_trace__("v", v)` when compiled under debug mode.
//! The label could be customized, which `fire::dbg!("arg_v", v)` would expand to `__firedbg_trace__("arg_v", v)`.
//! In release mode, it would expand to an expression passing through the value, i.e. `{ v }`.
//!
//! Note that the function passes through the ownership of the variable, just like the [`std::dbg!`](https://doc.rust-lang.org/std/macro.dbg.html) macro.
//!
//! ```ignore
//! fn __firedbg_trace__<T>(name: &'static str, v: T) -> T { v }
//! ```
pub mod fire {
    #[macro_export]
    #[cfg(debug_assertions)]
    macro_rules! dbg {
        ($t:expr, $v:expr) => {
            firedbg_lib::__firedbg_trace__($t, $v);
        };
        ($v:expr) => {
            firedbg_lib::__firedbg_trace__(std::stringify!($v), $v);
        };
    }

    #[macro_export]
    #[cfg(not(debug_assertions))]
    macro_rules! dbg {
        ($t:expr, $v:expr) => {{
            $v
        }};
        ($v:expr) => {{
            $v
        }};
    }

    pub use dbg;
}

#[cfg(debug_assertions)]
#[allow(unused_variables)]
pub fn __firedbg_trace__<T>(name: &'static str, v: T) -> T {
    v
}
