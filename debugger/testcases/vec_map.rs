fn square(i: i32) -> i32 {
    i * i
}

fn main() {
    let array: Vec<i32> = (0..10).collect();
    let array: Vec<i32> = array.into_iter().map(square).collect();
    dbg!(array);
}
