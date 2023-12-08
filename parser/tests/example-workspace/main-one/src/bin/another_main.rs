use main_one::main_one_common_fn;
use quick_sort::run_quick_sort;

fn main() {
    main_one_another_main();
    main_one_common_fn();

    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    run_quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
}

fn main_one_another_main() -> i32 {
    1
}
