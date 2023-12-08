use main_one::main_one_common_fn;
use quick_sort::run_quick_sort;

#[test]
fn main() {
    main_fn();
    main_one_common_fn();

    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    run_quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
}

fn main_fn() -> i32 {
    1
}

#[test]
fn main2() {
    main2_fn();
    assert!(false);
}

fn main2_fn() -> i32 {
    2
}
