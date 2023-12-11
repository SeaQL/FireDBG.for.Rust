struct Wrapper<T>(T);

struct RefWrapper<'a, T>(&'a T);

fn borrow<T: ?Sized>(v: &T) -> &T {
    &v
}

fn slice<T>(v: &[T], s: usize, e: usize) -> &[T] {
    &v[s..e]
}

fn u8() -> [u8; 10] {
    [1,2,3,4,5,6,7,8,9,10]
}

fn i8() -> [i8; 1] {
    [1]
}

fn u16() -> [u16; 6] {
    [1,2,3,4,5,6]
}

fn i16() -> [i16; 1] {
    [1]
}

fn u32() -> [u32; 4] {
    [1,2,3,4]
}

fn i32() -> [i32; 1] {
    [1]
}

fn u64() -> [u64; 3] {
    [1,2,3]
}

fn i64() -> [i64; 1] {
    [1]
}

fn u128() -> [u128; 3] {
    [1,2,3]
}

fn i128() -> [i128; 1] {
    [1]
}

fn f32() -> [f32; 3] {
    [1.0,2.0,3.0]
}

fn f64() -> [f64; 1] {
    [1.0]
}

fn usize() -> [usize; 3] {
    [1,2,3]
}

fn isize() -> [isize; 1] {
    [1]
}

fn str() -> [&'static str; 3] {
    ["1","2","3"]
}

fn wrapped() -> Wrapper<[u8; 9]> {
    Wrapper([9,8,7,6,5,4,3,2,1])
}

fn wrapped_ref(v: &[u8; 9]) -> RefWrapper<[u8; 9]> {
    RefWrapper(v)
}

fn multi() -> [[u8; 3]; 3] {
    [[1,2,3],[4,5,6],[7,8,9]]
}

fn multi_w() -> Wrapper<[[u8; 3]; 3]> {
    Wrapper([[9,8,7],[6,5,4],[3,2,1]])
}

fn multi_ref_w(v: &[[u8; 3]; 3]) -> RefWrapper<[[u8; 3]; 3]> {
    RefWrapper(v)
}

fn main() {
    let _u8 = u8(); borrow(&_u8); borrow(&_u8[1.._u8.len()-1]);
    let _i8 = i8(); borrow(&_i8); borrow(&_i8[..]);
    let _u16 = u16(); borrow(&_u16); borrow(&_u16[.._u16.len()-1]);
    let _i16 = i16(); borrow(&_i16); borrow(&_i16[1..]);
    let _u32 = u32(); borrow(&_u32); borrow(&_u32[2..=2]);
    let _i32 = i32(); borrow(&_i32); borrow(&_i32[1..]);
    let _u64 = u64(); borrow(&_u64); borrow(&_u64[1..]);
    let _i64 = i64(); borrow(&_i64); borrow(&_i64[1..]);
    let _u128 = u128(); borrow(&_u128); borrow(&_u128[1..]);
    let _i128 = i128(); borrow(&_i128); borrow(&_i128[1..]);
    let _f32 = f32(); borrow(&_f32); borrow(&_f32[1..]);
    let _f64 = f64(); borrow(&_f64); borrow(&_f64[1..]);
    let _usize = usize(); borrow(&_usize); borrow(&_usize[1..]);
    let _isize = isize(); borrow(&_isize); borrow(&_isize[1..]);
    let _str = str(); borrow(&_str); borrow(&_str[1..]);
    let _wrapped = wrapped(); borrow(&_wrapped);
    let _wrapped_ref = wrapped_ref(&_wrapped.0);
    let _multi = multi(); borrow(&_multi);
    let _multi_w = multi_w(); borrow(&_multi_w);
    let _multi_ref_w = multi_ref_w(&_multi_w.0);
    let _vec = vec![1,2,3,4,5,6,7,8];
    borrow(&_vec[1..2]);
    slice(&_vec, 1, 2);
    slice(&_u8, 0, 0);
}