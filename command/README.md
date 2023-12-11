## FireDBG Command Line Interface

`firedbg-cli` is a CLI to invoke all FireDBG operations.

### Cargo Workspace

The `firedbg` command can only act on [Cargo Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html). If you have a simple dependency free rust file, you still need to put it under a cargo workspace for `firedbg` to work properly.

There are two ways to tell `firedbg` where is the root directory of a cargo workspace:

1. By default, the current directory will be the root directory of a cargo workspace
2. Or, overriding it with `--workspace-root` option, i.e. `firedbg --workspace-root <WORKSPACE-ROOT>`

### Common Subcommands

- `cache`: Parse all `.rs` source files in the current workspace
- `clean`: Cleanup the `firedbg/` folder
- `list-target`: List all runnable targets
- `run`: Run a binary target with debugging enabled
- `example`: Run an example with debugging enabled
- `test`: Run an integrated test with debugging enabled
- `unit-test`: Run a unit test with debugging enabled
- `index`: Run indexer on the latest run and save it as a `.sqlite` db file
- `list-run`: List all `firedbg` runs
- `open`: Open debugger view in VS Code
- `help`: Print help message or the help of the given subcommand(s)

You can get the help messages by appending the `--help` flag.

### The `firedbg.toml` Config File

By default FireDBG will only trace the function calls of the debugging package. If you want to trace other packages in your local workspace, you will need to create a `firedbg.toml` config file on your workspace root.

```toml
[workspace.members]
quicksort = { trace = "full" }
# Syntax: <PACKAGE> = { trace = "<full | none>" }
```
