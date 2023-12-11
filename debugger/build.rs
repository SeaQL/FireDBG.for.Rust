// Statically import `rustc_version` method into scope
include!("src/version.rs");

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let lldb_rel_dir = std::env::var("LLDB_REL_DIR").unwrap_or_else(|_| "lldb".to_owned());
    // At compile/link time, it is relative to workspace root
    println!(r"cargo:rustc-link-search={lldb_rel_dir}/lib");
    // At runtime, it is relative to the current directory
    if target_os == "linux" {
        println!(r"cargo:rustc-env=LD_LIBRARY_PATH=../{lldb_rel_dir}/lib");
    } else if target_os == "macos" {
        println!(r"cargo:rustc-env=DYLD_FALLBACK_LIBRARY_PATH=../{lldb_rel_dir}/lib");
    }
    println!(r"cargo:rustc-env=RUSTC_VERSION={}", rustc_version());
}
