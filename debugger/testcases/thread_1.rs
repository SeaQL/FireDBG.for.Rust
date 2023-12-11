fn tick(i: usize) {
    println!("tick {i}");
}

fn thread_1() {
    for i in 0..10 {
        tick(i);
    }
}

fn run() {
    let handler = std::thread::spawn(thread_1);
    handler.join().unwrap();
}

fn main() {
    run();
}