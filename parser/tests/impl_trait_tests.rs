mod common;

pub use firedbg_rust_parser::*;
pub use pretty_assertions::assert_eq;

#[test]
fn parse_impl_trait() -> anyhow::Result<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common/impl_trait.rs");
    let mut breakpoints = parse_file(path)?;
    breakpoints.pop(); // Pop the last `fn get_breakpoints` breakpoint
    assert_eq!(breakpoints, common::impl_trait::get_breakpoints());
    Ok(())
}
