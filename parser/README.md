## FireDBG Source Parser for Rust

Based on [`syn`](https://github.com/dtolnay/syn).

`firedbg-rust-parser` is a Rust source code parser. It can parse a Rust source file, walk the abstract syntax tree of Rust, then produce a list of breakpoints for the debugger to pause the program at the beginning of every function call.

### Walking the AST

We will walk the Rust AST, [`syn::Item`](https://docs.rs/syn/latest/syn/enum.Item.html), and collect all forms of function / method:

1. Free standalone function, [`syn::Item::Fn`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Fn)
2. Impl function, [`syn::Item::Impl`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Impl)
3. Trait default function, [`syn::Item::Trait`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Trait)
4. Impl trait function, [`syn::Item::Impl`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Impl)
5. Nested function, walking the [`syn::Item`](https://docs.rs/syn/latest/syn/enum.Item.html) recursively
6. Function defined inside inline module, [`syn::Item::Mod`](https://docs.rs/syn/latest/syn/enum.Item.html#variant.Mod)

### Breakable Span

A span is a region of source code, denoted by a ranged line and column number tuple, along with macro expansion information.
It allows the debugger to set a breakpoint at the correct location. The debugger will set the breakpoint either at the start or the end of the breakable span.

```rust
fn func() -> i32 {
/*                ^-- Start of Breakable Span: (Line 1, Column 19)  */
    let mut i = 0;
/*  ^-- End of Breakable Span: (Line 3, Column 5)  */
    for _ in (1..10) {
        i += 1;
    }
    i
}
```

### Ideas

The current implementation is rudimentary, but we get exactly what we need. We considered embedding Rust Analyzer, for a few advantages: 1) to get the fully-qualified type names
2) to traverse the static call graph. The problem is resource usage: we'd end up running the compiler frontend thrice (by cargo, by language server, by firedbg).
