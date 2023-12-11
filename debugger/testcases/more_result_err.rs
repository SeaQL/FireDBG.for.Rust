use std::hint::black_box as bb;

fn capture<T>(i: f64, v: T) -> T {
    bb(v)
}

fn main() {
    let mut v: Result<(), ()> = capture(1.1, Err(())); dbg!(v);
    v = Ok(()); v = capture(1.2, v); dbg!(v);
    
    let mut v: Result<(), bool> = capture(2.1, Err(false)); dbg!(v);
    v = Err(true); v = capture(2.2, v); dbg!(v);
    v = Ok(()); v = capture(2.3, v); dbg!(v);
    
    let mut v: Result<(), i8> = capture(3.1, Err(-22)); dbg!(v);
    v = Err(22); v = capture(3.2, v); dbg!(v);
    v = Ok(()); v = capture(3.3, v); dbg!(v);
    
    let mut v: Result<(), u8> = capture(4.1, Err(250)); dbg!(v);
    v = Ok(()); v = capture(4.2, v); dbg!(v);
    
    let mut v: Result<(), i16> = capture(5.1, Err(-22222)); dbg!(v);
    v = Err(22222); v = capture(5.2, v); dbg!(v);
    v = Ok(()); v = capture(5.3, v); dbg!(v);
    
    let mut v: Result<(), u16> = capture(6.1, Err(65432)); dbg!(v);
    v = Ok(()); v = capture(6.2, v); dbg!(v);
    
    let mut v: Result<(), i32> = capture(7.1, Err(-222_222)); dbg!(v);
    v = Err(222_222); v = capture(7.2, v); dbg!(v);
    v = Ok(()); v = capture(7.3, v); dbg!(v);
    
    let mut v: Result<(), u32> = capture(8.1, Err(432_432)); dbg!(v);
    v = Ok(()); v = capture(8.2, v); dbg!(v);
    
    let mut v: Result<(), i64> = capture(9.1, Err(-22_222_222_222)); dbg!(v);
    v = Err(22_222_222_222); v = capture(9.2, v); dbg!(v);
    v = Ok(()); v = capture(9.3, v); dbg!(v);
    
    let mut v: Result<(), u64> = capture(10.1, Err(23_232_232_232)); dbg!(v);
    v = Ok(()); v = capture(10.2, v); dbg!(v);
    
    let mut v: Result<(), isize> = capture(11.1, Err(-22_222_222_222)); dbg!(v);
    v = Err(22_222_222_222); v = capture(11.2, v); dbg!(v);
    v = Ok(()); v = capture(11.3, v); dbg!(v);
    
    let mut v: Result<(), usize> = capture(12.1, Err(23_232_232_232)); dbg!(v);
    v = Ok(()); v = capture(12.2, v); dbg!(v);
    
    let mut v: Result<(), i128> = capture(13.1, Err(-22_222_222_222_222_222_222)); dbg!(v);
    v = Err(22_222_222_222_222_222_222); v = capture(13.2, v); dbg!(v);
    v = Ok(()); v = capture(13.3, v); dbg!(v);
    
    let mut v: Result<(), u128> = capture(14.1, Err(33_333_333_333_333_333_333)); dbg!(v);
    v = Ok(()); v = capture(14.2, v); dbg!(v);
    
    let mut v: Result<(), f32> = capture(15.1, Err(111.111)); dbg!(v);
    v = Err(-111.111); v = capture(15.2, v); dbg!(v);
    v = Ok(()); v = capture(15.3, v); dbg!(v);
    
    let mut v: Result<(), f64> = capture(16.1, Err(222.222)); dbg!(v);
    v = Err(-222.222); v = capture(16.2, v); dbg!(v);
    v = Ok(()); v = capture(16.3, v); dbg!(v);
    
    let mut v: Result<(), &'static str> = capture(17.1, Err("hello")); dbg!(v);
    v = Ok(()); v = capture(17.2, v); dbg!(v);
    
    let mut v: Result<(), char> = capture(18.1, Err('🦀')); dbg!(v);
    v = Ok(()); v = capture(18.2, v); dbg!(v);
}