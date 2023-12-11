fn iter(mut i: i32) {
    loop {
        if i <= 0 {
            break;
        }
        println!("{i}");
        i -= 1;
    }
}

fn run() {
    iter(3);
    iter(2);
}

fn main() {
    run();
}
