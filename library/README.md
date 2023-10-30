# FireDBG Support Library

## `fire::dbg!`

This macro allows you to capture the value of a variable via runtime inspection in FireDBG.
Usage example:

```rust
use firedbg_lib::fire;

fn some_fn(v: i32) -> i32 {
    fire::dbg!(v) + 1
}
```

Which `fire::dbg!(v)` would expand to `__firedbg_trace__("v", v)` when compiled under debug mode.
In release mode, it would expand to an expression, i.e. `{ v }`.

Note that the function passes through the ownership of the variable, like the [`std::dbg!`] macro.

```rust
fn __firedbg_trace__<T>(v: T) { v }
```
