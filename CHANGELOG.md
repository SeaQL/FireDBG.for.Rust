# Changelog

## 1.80.0 - 2024-07-26

### Versions

+ `1.80.0-beta.1`: 2024-07-22

## 1.79.0 - 2024-06-14

### Versions

+ `1.79.0-beta.1`: 2024-05-30

## 1.78.0 - 2024-05-02

### Versions

+ `1.78.0-beta.1`: 2024-04-23

## 1.77.1 - 2024-04-19

### Bug Fixes

- debugger: fix memory layout probing https://github.com/SeaQL/FireDBG.for.Rust/pull/41

## 1.77.0 - 2024-03-28

### Versions

+ `1.77.0-beta.1`: 2024-03-25

### New Features

- debugger: trace allocations (Box, Arc, Rc)

## 1.76.0 - 2024-02-08

### Versions

+ `1.76.0-beta.1`: 2024-02-06

## 1.75.1 - 2024-01-08

### Enhancements

- parser: parse `required-features` from Cargo Metadata
- command: compile Rust target with `required-features`
- command: selectively list binary, example, integration test and unit test targets

## 1.75.0 - 2023-12-28

### New Features

- command: new `targets` section in `firedbg.toml` to provide params to debugg targets
```toml
[[targets]]
# `quicksort/src/bin/quicksort.rs` (Binary)
name = "quicksort_100"
target.type = "binary"
target.name = "quicksort"
argv = ["100", "--seed", "1212"]
```

### Enhancements

- debugger: now streams relative path to workspace root in `SourceFile.path`

## 1.74.2 - 2023-12-14

### Enhancements

- Prettify `OsString`
- Better rendering of `Vec<String>` and `Vec<Vec<String>>`

## 1.74.1 - 2023-12-12

### Bug Fixes

- debugger: fix package config parsing https://github.com/SeaQL/FireDBG.for.Rust/pull/5

## 1.74.0 - 2023-12-11

- Initial release
