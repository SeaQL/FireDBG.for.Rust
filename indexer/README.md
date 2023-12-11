## FireDBG Event Indexer

`firedbg-stream-indexer` is a streaming indexer. It can stream events from `.firedbg.ss` files, index them in real-time, and write updates to `.sqlite` incrementally.

There are 4 event types:

| Event Code | Event Type | Description |
|:----------:|:----:|:-----------:|
| `B` | Breakpoint | e.g. a breakpoint hit by `fire::dbg!`
| `P` | Panic | Program panic |
| `F` | Function Call | - |
| `R` | Function Return | - |

The indexer reconstructs the call stack for each thread from the event stream, and write a `parent_frame_id` for each `F` event.

The indexer also deserializes the value blobs and translates them into JSON. The JSON is then transformed into pretty-printed Rust-like value strings:

```rust
Value Blob -> RValue -> Lifted RValue -> Pretty Print
```

The database schema can be found under [`indexer/src/entity/`](https://github.com/SeaQL/FireDBG.for.Rust/tree/main/indexer/src/entity/), which is defined by a set of SeaORM entities.

Highly recommend you to install a SQLite extension for VS Code. You can find some sample indexes in the [Testbench](https://github.com/SeaQL/FireDBG.Rust.Testbench).
