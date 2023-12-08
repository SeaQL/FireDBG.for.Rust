mod conquer;
mod divide;

use divide::quick_sort;
use std::fmt::Debug;

pub fn run_quick_sort<T: PartialOrd + Debug>(arr: &mut [T]) {
    let len = arr.len();
    quick_sort(arr, 0, (len - 1) as isize);
}

#[cfg(test)]
mod test {
    use crate::run_quick_sort;

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
