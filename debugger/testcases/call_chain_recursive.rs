fn chain(i: usize) -> usize {
    if i == 0 {
        return i;
    }
    return i + chain(i - 1);
}

fn main() {
    assert_eq!(chain(10), 55);
}