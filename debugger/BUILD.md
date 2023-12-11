# How to build

## Setup

The following steps are experimented on Ubuntu 22.04.

Dependencies:

```
apt install clang libc++-dev libc++abi-dev
```

For the lldb binary, obtain codelldb, then copy or link the content of `/lldb/` into `/lldb/`.

So the project directory would look like the following:

```
├── debugger/
|   └── testcases/
|       └── cars.o*
└── lldb/
    ├── bin/
    |   └── lldb-server*
    └── lib/
        └── liblldb.so
```

Make sure the `*` files have executable permissions, i.e. `chmod +x`.

### From GitHub Release

Download from https://github.com/vadimcn/codelldb/releases.

We are using `v1.10.0`.

### From vscode

```
~/.vscode/extensions/vadimcn.vscode-lldb-1.10.0
```

## Build

Under the directory `debugger`, run `cargo build` to build it.

## Test

Run `cargo test --test '*' -- --nocapture` to execute tests.

```log
running 1 test
Debugger (instance: "debugger_1", id: 1)
fn_call.o
SBProcess: pid = 31594, state = exited, threads = 1, executable = fn_call.o
FunctionCall `fn_call::main`
FunctionCall `fn_call::hello`
world = {"type":"Struct","typename":"fn_call::World","fields":{"nth":{"type":"Prim","typename":"i32","value":99}}}
FunctionCall `<fn_call::World as core::fmt::Display>::fmt`
self  = {"type":"Struct","typename":"fn_call::World","fields":{"nth":{"type":"Prim","typename":"i32","value":99}}}
test main ... ok
```

## Troubleshooting

### `"cannot find -lc++abi" No such file or directory`

Possible solution:

```
sudo apt-get install libc++abi-dev
```

### `c++: error: unrecognized command-line option ‘-stdlib=libc++’`

Possible solution:

Link c++ to clang instead of g++

```
sudo update-alternatives --config c++
```
