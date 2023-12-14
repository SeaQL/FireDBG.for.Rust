## Installing FireDBG

### Installer

Run the following command to install the prebuilt binaries of FireDBG.

```shell
curl https://raw.githubusercontent.com/SeaQL/FireDBG.for.Rust/main/install.sh -sSf | sh
```

FireDBG binaries will be installed in `~/.cargo/bin` and a debugger self test will be conducted to verify the installation. Expect to see:

```shell
info: completed FireDBG self tests
```

In case you got error messages when performing self test, read [Troubleshooting Guide](https://github.com/SeaQL/FireDBG.for.Rust/blob/main/Troubleshooting.md) for the solution of common errors.

### Manual Installation

If you saw `FireDBG: no precompiled binaries available for OS: ...`, you can manually download a prebuilt binaries of compatible OS.

1. Go to the [FireDBG release](https://github.com/SeaQL/FireDBG.for.Rust/releases) page

2. Download a prebuilt binaries of compatible OS

    ```shell
    # Download the prebuilt binaries
    curl -sSfL "https://github.com/SeaQL/FireDBG.for.Rust/releases/download/1.74.1/x86_64-ubuntu22.04.tar.gz" -o "x86_64-ubuntu22.04.tar.gz"

    # General form
    curl -sSfL "https://github.com/SeaQL/FireDBG.for.Rust/releases/download/<LATEST_VERSION>/<ARC>-<OS>.tar.gz" -o "<ARC>-<OS>.tar.gz"
    ```

3. Unzip the `.tar.gz`

    ```shell
    # Unzip
    mkdir -p "x86_64-ubuntu22.04" && tar xf "x86_64-ubuntu22.04.tar.gz" --strip-components 1 -C "x86_64-ubuntu22.04"

    # General form
    mkdir -p "<ARCH>-<OS>" && tar xf "<ARCH>-<OS>.tar.gz" --strip-components 1 -C "<ARCH>-<OS>"
    ```

4. Copy FireDBG binaries into `$HOME/.cargo/bin`

    ```shell
    # Copy
    mkdir -p "$HOME/.cargo/bin" && cp -r x86_64-ubuntu22.04/* "$HOME/.cargo/bin/"

    # General form
    mkdir -p "$HOME/.cargo/bin" && cp -r <ARCH>-<OS>/* "$HOME/.cargo/bin/"
    ```

5. Perform debugger self tests

    ```shell
    cd "$HOME/.cargo/bin/firedbg-lib/debugger-self-test"
    rm -f *.firedbg.ss
    firedbg run debugger_self_test --output output.firedbg.ss
    firedbg-indexer --input output.firedbg.ss validate --json expected_data.json && echo "info: completed FireDBG self tests"
    ```
