use quick_sort::run_quick_sort;

fn main() {
    quick_sort_main();

    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    run_quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
}

fn quick_sort_main() -> i32 {
    1
}
