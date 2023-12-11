#[inline(never)]
fn tick(i: usize) {
    println!("[{:?}] tick {i}", std::thread::current().id());
}

fn runner(s: isize) {
    for i in (0..10).step_by(s as usize) {
        tick(i);
        std::thread::sleep(std::time::Duration::from_millis(s as u64));
    }
}

fn run(s: isize) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || runner(s))
}

fn main() {
    let threads = [run(1), run(2)];
    for handle in threads {
        handle.join().unwrap();
    }
}