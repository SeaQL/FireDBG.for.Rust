#[inline(never)]
fn vec_i32(n: usize) -> Vec<i32> {
    let mut vec = Vec::new();
    for i in 1..=n {
        vec.push(i as i32);
    }
    vec
}

#[inline(never)]
fn vec_i32_iter(n: usize) -> Vec<i32> {
    (1..=n).map(|i| i as i32).collect()
}

#[inline(never)]
fn vec_char_iter(n: usize) -> Vec<char> {
    (0..n).map(|i| std::char::from_u32(b'a' as u32 + i as u32).unwrap()).collect()
}

#[inline(never)]
fn vec_char_iter_never_works(n: usize) -> Vec<char> {
    // sadly the object is not reachable
    let items = vec_i32_iter(3);
    items.iter().map(|i| std::char::from_u32(b'a' as u32 + (*i) as u32 - 1).unwrap()).collect()
}

fn main() {
    assert!(vec_i32(0).is_empty());
    assert_eq!(vec_i32(1).len(), 1);
    assert_eq!(vec_i32(2).len(), 2);

    assert!(vec_i32_iter(0).is_empty());
    assert_eq!(vec_i32_iter(2).len(), 2);
    assert_eq!(vec_i32_iter(3).len(), 3);

    assert_eq!(vec_char_iter(3).len(), 3);
    // assert_eq!(vec_char_iter_never_works(3).len(), 3);
}
