pub fn partition<T: PartialOrd>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
}

#[cfg(test)]
mod test {
    use crate::run_quick_sort;

    fn test_partition_fn() -> i32 {
        1
    }

    #[test]
    fn test_partition() {
        test_partition_fn();

        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        run_quick_sort(&mut numbers);
        println!("After:  {:?}\n", numbers);
    }

    fn test_partition2_fn() -> i32 {
        2
    }

    #[test]
    fn test_partition2() {
        test_partition2_fn();

        assert!(false);
    }
}
