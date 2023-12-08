fn main() {
    let args: Vec<_> = std::env::args().collect();
    let workspace = firedbg_rust_parser::parse_workspace(&args[1]);
    println!("{:#?}", workspace);
}
