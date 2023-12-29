# Changelog

## 1.75.0 - 2023-12-28

### New Features

- command: new `targets` section in `firedbg.toml` to provide paramsto debugg targets
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
