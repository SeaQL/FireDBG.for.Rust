fn iter(mut i: i32) {
    for n in (0..i) {
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
