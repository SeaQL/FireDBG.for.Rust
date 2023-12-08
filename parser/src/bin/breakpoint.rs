fn main() {
    let args: Vec<_> = std::env::args().collect();
    let files = firedbg_rust_parser::parse_directory(&args[1]);
    println!("{:#?}", files);
}
