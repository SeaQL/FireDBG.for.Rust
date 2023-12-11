fn no_prolog(a: i32) {
    println!("a = {a}");
}

fn prolog(a: i32, b: i32, c: i32) -> Result<(), ()> {
    let (a, b, c) = (dice(a), dice(b), dice(c));
    let a = a?;
    let b = b?;
    let c = c?;
    println!("a = {a}; b = {b}; c = {c}");
    Ok(())
}

fn dice(i: i32) -> Result<i32, ()> {
    std::hint::black_box(Ok(i))
}

fn main() {
    no_prolog(12345678);
    prolog(1234, 5678, 12345678).unwrap();
}