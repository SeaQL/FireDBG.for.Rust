fn hello(i: i32) -> String {
    std::hint::black_box(format!("hello {i}"))
}

fn world() -> String {
    hello(11)
}

fn main() {
    println!("{}", hello(22));
    println!("{}", world());
}