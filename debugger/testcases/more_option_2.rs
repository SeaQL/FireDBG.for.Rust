use std::hint::black_box as bb;

fn capture<T>(i: f64, v: T) -> T {
    bb(v)
}

fn main() {
    let mut v: Option<()> = capture(1.1, Some(())); dbg!(v);
    v = None; v = capture(1.2, v); dbg!(v);
    
    let mut v: Option<bool> = capture(2.1, Some(false)); dbg!(v);
    v = Some(true); v = capture(2.2, v); dbg!(v);
    v = None; v = capture(2.3, v); dbg!(v);
    
    let mut v: Option<i8> = capture(3.1, Some(-22)); dbg!(v);
    v = Some(22); v = capture(3.2, v); dbg!(v);
    v = None; v = capture(3.3, v); dbg!(v);
    
    let mut v: Option<u8> = capture(4.1, Some(250)); dbg!(v);
    v = None; v = capture(4.2, v); dbg!(v);
    
    let mut v: Option<i16> = capture(5.1, Some(-22222)); dbg!(v);
    v = Some(22222); v = capture(5.2, v); dbg!(v);
    v = None; v = capture(5.3, v); dbg!(v);
    
    let mut v: Option<u16> = capture(6.1, Some(65432)); dbg!(v);
    v = None; v = capture(6.2, v); dbg!(v);
    
    let mut v: Option<i32> = capture(7.1, Some(-222_222)); dbg!(v);
    v = Some(222_222); v = capture(7.2, v); dbg!(v);
    v = None; v = capture(7.3, v); dbg!(v);
    
    let mut v: Option<u32> = capture(8.1, Some(432_432)); dbg!(v);
    v = None; v = capture(8.2, v); dbg!(v);
    
    let mut v: Option<i64> = capture(9.1, Some(-22_222_222_222)); dbg!(v);
    v = Some(22_222_222_222); v = capture(9.2, v); dbg!(v);
    v = None; v = capture(9.3, v); dbg!(v);
    
    let mut v: Option<u64> = capture(10.1, Some(23_232_232_232)); dbg!(v);
    v = None; v = capture(10.2, v); dbg!(v);
    
    let mut v: Option<isize> = capture(11.1, Some(-22_222_222_222)); dbg!(v);
    v = Some(22_222_222_222); v = capture(11.2, v); dbg!(v);
    v = None; v = capture(11.3, v); dbg!(v);
    
    let mut v: Option<usize> = capture(12.1, Some(23_232_232_232)); dbg!(v);
    v = None; v = capture(12.2, v); dbg!(v);
    
    let mut v: Option<i128> = capture(13.1, Some(-22_222_222_222_222_222_222)); dbg!(v);
    v = Some(22_222_222_222_222_222_222); v = capture(13.2, v); dbg!(v);
    v = None; v = capture(13.3, v); dbg!(v);
    
    let mut v: Option<u128> = capture(14.1, Some(33_333_333_333_333_333_333)); dbg!(v);
    v = None; v = capture(14.2, v); dbg!(v);
    
    let mut v: Option<f32> = capture(15.1, Some(111.111)); dbg!(v);
    v = Some(-111.111); v = capture(15.2, v); dbg!(v);
    v = None; v = capture(15.3, v); dbg!(v);
    
    let mut v: Option<f64> = capture(16.1, Some(222.222)); dbg!(v);
    v = Some(-222.222); v = capture(16.2, v); dbg!(v);
    v = None; v = capture(16.3, v); dbg!(v);
    
    let mut v: Option<&'static str> = capture(17.1, Some("hello")); dbg!(v);
    v = None; v = capture(17.2, v); dbg!(v);
    
    let mut v: Option<char> = capture(18.1, Some('🦀')); dbg!(v);
    v = None; v = capture(18.2, v); dbg!(v);
}