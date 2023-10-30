//! # FireDBG Support Library
//!
//! ## `fire::dbg!`
//!
//! This macro allows you to capture the value of a variable via runtime inspection in FireDBG.
//! Usage example:
//!
//! ```
//! use firedbg_lib::fire;
//!
//! fn some_fn(v: i32) -> i32 {
//!     fire::dbg!(v) + 1
//! }
//! ```
//!
//! Which `fire::dbg!(v)` would expand to `__firedbg_trace__("v", v)` when compiled under debug mode.
//! In release mode, it would expand to an expression, i.e. `{ v }`.
//!
//! Note that the function passes through the ownership of the variable, like the [`std::dbg!`] macro.
//!
//! ```ignore
//! fn __firedbg_trace__<T>(v: T) { v }
//! ```
pub mod fire {
    #[macro_export]
    #[cfg(debug_assertions)]
    macro_rules! dbg {
        ($v:expr) => {
            firedbg_lib::__firedbg_trace__(std::stringify!($v), $v);
        };
    }

    #[macro_export]
    #[cfg(not(debug_assertions))]
    macro_rules! dbg {
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
