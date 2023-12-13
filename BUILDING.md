## Building FireDBG

1. Launch Docker

2. Build the Docker image of the host OS

    ```shell
    # Run this command at the project root
    docker build --no-cache -t firedbg-ubuntu22.04 -f build/Dockerfile.ubuntu22.04 .

    # The general form
    docker build --no-cache -t firedbg-<OS_NAME> -f build/Dockerfile.<OS_NAME> .
    ```

3. Run the Docker image we just built

    ```shell
    # Run this command at the project root
    docker run --name firedbg-ubuntu22.04 --rm -it -v $(pwd):/FireDBG.for.Rust firedbg-ubuntu22.04

    # The general form
    docker run --name firedbg-<OS_NAME> --rm -it -v $(pwd):/FireDBG.for.Rust firedbg-<OS_NAME>
    ```

4. FireDBG requires [`codelldb`](https://github.com/vadimcn/codelldb) binaries when building from source, the binaries should be placed under `FireDBG.for.Rust/lldb` directory

    ```shell
    # Download the `vsix` from GitHub
    curl -SfL "https://github.com/vadimcn/codelldb/releases/download/v1.10.0/codelldb-x86_64-linux.vsix" -o "codelldb-x86_64-linux.vsix"

    # Unzip it
    unzip -q "codelldb-x86_64-linux.vsix" -d "codelldb-x86_64-linux"

    # Place the binaries under `FireDBG.for.Rust/lldb` directory
    mv "codelldb-x86_64-linux/extension/lldb" "FireDBG.for.Rust/lldb"
    ```

5. Building FireDBG binaries

    ```shell
    cd FireDBG.for.Rust

    cargo build --manifest-path "command/Cargo.toml"
    cargo build --manifest-path "indexer/Cargo.toml"
    cargo build --manifest-path "debugger/Cargo.toml"
    ```
