use std::collections::{HashMap, HashSet};

struct Point<T> {
    x: T,
    y: T,
}

fn hash_map_it<E, F>(v: HashMap<E, F>) {
    std::hint::black_box(v);
}
fn hash_set_it<E>(v: HashSet<E>) {
    std::hint::black_box(v);
}
fn main() {
    let map: HashMap<char, i32> = [('a', 1), ('b', 2), ('c', 3), ('d', 4)]
        .into_iter()
        .collect();
    hash_map_it(map);
    let map: HashMap<u8, u8> = [(b'a', 1), (b'b', 2), (b'c', 3), (b'd', 4)]
        .into_iter()
        .collect();
    hash_map_it(map);
    let map: HashMap<&str, u32> = [
        ("aa", 111_111),
        ("bb", 222_222),
        ("cc", 333_333),
        ("dd", 444_444),
    ]
    .into_iter()
    .collect();
    hash_map_it(map);
    let map: HashMap<&str, u64> = [
        ("aa", 111_111_111),
        ("bb", 222_222_222),
        ("cc", 333_333_333),
        ("dd", 444_444_444),
    ]
    .into_iter()
    .collect();
    hash_map_it(map);
    let map: HashMap<char, i64> = [
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
    ]
    .into_iter()
    .collect();
    hash_map_it(map);
    let map: HashMap<char, Point<i32>> = [
        ('a', Point { x: 10, y: 11 }),
        ('b', Point { x: 20, y: 22 }),
        ('c', Point { x: 30, y: 33 }),
        ('d', Point { x: 40, y: 44 }),
    ]
    .into_iter()
    .collect();
    hash_map_it(map);
    let map: HashMap<u8, Point<i32>> = [
        (b'a', Point { x: 10, y: 11 }),
        (b'b', Point { x: 20, y: 22 }),
        (b'c', Point { x: 30, y: 33 }),
        (b'd', Point { x: 40, y: 44 }),
    ]
    .into_iter()
    .collect();
    hash_map_it(map);
    let map: HashMap<u8, Point<f32>> = [
        (b'a', Point { x: 1.0, y: 1.1 }),
        (b'b', Point { x: 2.0, y: 2.2 }),
        (b'c', Point { x: 3.0, y: 3.3 }),
        (b'd', Point { x: 4.0, y: 4.4 }),
    ]
    .into_iter()
    .collect();
    hash_map_it(map);
    let map: HashSet<u32> = [1, 2, 3, 4].into_iter().collect();
    hash_set_it(map);
    let map: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
    hash_set_it(map);
    let map: HashSet<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().collect();
    hash_set_it(map);
    let map: HashMap<String, String> = [
        ("aa", "aaaa"),
        ("bb", "bbbb"),
        ("cc", "cccc"),
        ("dd", "dddd"),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_owned(), b.to_owned()))
    .collect();
    hash_map_it(map);
    let map: HashSet<i32> = (1..=1000).collect();
    hash_set_it(map);
}
