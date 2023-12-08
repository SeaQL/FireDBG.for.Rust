use crate::conquer::partition;
use std::fmt::Debug;

pub fn quick_sort<T: PartialOrd + Debug>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        println!("Before: {:?}", arr);
        let p = partition(arr, low, high);
        println!("After:  {:?}\n", arr);
        quick_sort(arr, low, p - 1);
        quick_sort(arr, p + 1, high);
    }
}
