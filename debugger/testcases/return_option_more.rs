fn usize_pair(i: i32) -> Option<(usize, usize)> {
    if i == 0 {
        None
    } else {
        Some((1234, 5678))
    }
}

fn main() {
    assert!(usize_pair(0).is_none());
    assert!(usize_pair(1).is_some());
}
