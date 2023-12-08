pub fn main_one_common_fn() -> i32 {
    0
}

#[cfg(test)]
mod test {
    use quick_sort::run_quick_sort;

    #[test]
    fn test_quicksort() {
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        run_quick_sort(&mut numbers);
        println!("After:  {:?}\n", numbers);
    }

    #[test]
    fn test_quicksort2() {
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        run_quick_sort(&mut numbers);
        println!("After:  {:?}\n", numbers);
    }
}
