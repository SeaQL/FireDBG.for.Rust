use main_one::main_one_common_fn;
use quick_sort::run_quick_sort;
use rand::Rng;

fn main() {
    const TIMES: usize = 1;
    const VEC_SIZE: usize = 1_000;

    main_one_main();
    main_one_common_fn();

    for i in 0..TIMES {
        let mut rng = rand::thread_rng();

        let mut numbers: Vec<u64> = (0..VEC_SIZE).map(|_| rng.gen_range(0..1_000_000)).collect();

        println!("#{i} Sort numbers ascending");
        println!("Before: {:?}\n", numbers);
        run_quick_sort(&mut numbers);
        println!("After:  {:?}\n", numbers);
    }
}

fn main_one_main() -> i32 {
    1
}
