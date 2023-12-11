fn factorial(i: i32) -> i32 {
    if i == 0 {
        1
    } else {
        i * factorial(i - 1)
    }
}

fn main() {
    assert_eq!(factorial(5), 120);
}