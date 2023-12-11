struct Wrapper<T>(T);

fn u8_10() -> [u8; 10] {
    [1,2,3,4,5,6,7,8,9,10]
}

fn u8_9() -> [u8; 9] {
    [1,2,3,4,5,6,7,8,9]
}

fn u8_8() -> [u8; 8] {
    [1,2,3,4,5,6,7,8]
}

fn u8_6() -> [u8; 6] {
    [1,2,3,4,5,6]
}

fn u8_4() -> [u8; 4] {
    [1,2,3,4]
}

fn u8_2() -> [u8; 2] {
    [1,2]
}

fn u8_1() -> [u8; 1] {
    [1]
}

fn i8_1() -> [i8; 1] {
    [1]
}

fn u16_1() -> [u16; 1] {
    [1]
}

fn u16_2() -> [u16; 2] {
    [1,2]
}

fn u16_4() -> [u16; 4] {
    [1,2,3,4]
}

fn u16_5() -> [u16; 5] {
    [1,2,3,4,5]
}

fn u16_6() -> [u16; 6] {
    [1,2,3,4,5,6]
}

fn i16_1() -> [i16; 1] {
    [1]
}

fn i16_2() -> [i16; 2] {
    [1,2]
}

fn i16_4() -> [i16; 4] {
    [1,2,3,4]
}

fn i16_5() -> [i16; 5] {
    [1,2,3,4,5]
}

fn i16_6() -> [i16; 6] {
    [1,2,3,4,5,6]
}

fn u32_1() -> [u32; 1] {
    [1]
}

fn u32_2() -> [u32; 2] {
    [1,2]
}

fn u32_3() -> [u32; 3] {
    [1,2,3]
}

fn u32_4() -> [u32; 4] {
    [1,2,3,4]
}

fn i32_1() -> [i32; 1] {
    [1]
}

fn i32_2() -> [i32; 2] {
    [1,2]
}

fn i32_3() -> [i32; 3] {
    [1,2,3]
}

fn i32_4() -> [i32; 4] {
    [1,2,3,4]
}

fn u64_1() -> [u64; 1] {
    [1]
}

fn u64_2() -> [u64; 2] {
    [1,2]
}

fn u64_3() -> [u64; 3] {
    [1,2,3]
}

fn i64_1() -> [i64; 1] {
    [1]
}

fn i64_2() -> [i64; 2] {
    [1,2]
}

fn i64_3() -> [i64; 3] {
    [1,2,3]
}

fn u128_1() -> [u128; 1] {
    [1]
}

fn u128_2() -> [u128; 2] {
    [1,2]
}

fn u128_3() -> [u128; 3] {
    [1,2,3]
}

fn i128_1() -> [i128; 1] {
    [1]
}

fn i128_2() -> [i128; 2] {
    [1,2]
}

fn i128_3() -> [i128; 3] {
    [1,2,3]
}

fn f32_1() -> [f32; 1] {
    [1.0]
}

fn f32_2() -> [f32; 2] {
    [1.0,2.0]
}

fn f32_3() -> [f32; 3] {
    [1.0,2.0,3.0]
}

fn f64_1() -> [f64; 1] {
    [1.0]
}

fn f64_2() -> [f64; 2] {
    [1.0,2.0]
}

fn f64_3() -> [f64; 3] {
    [1.0,2.0,3.0]
}

fn usize_1() -> [usize; 1] {
    [1]
}

fn usize_2() -> [usize; 2] {
    [1,2]
}

fn usize_3() -> [usize; 3] {
    [1,2,3]
}

fn isize_1() -> [isize; 1] {
    [1]
}

fn isize_2() -> [isize; 2] {
    [1,2]
}

fn isize_3() -> [isize; 3] {
    [1,2,3]
}

fn str_1() -> [&'static str; 1] {
    ["1"]
}

fn str_2() -> [&'static str; 2] {
    ["1","2"]
}

fn str_3() -> [&'static str; 3] {
    ["1","2","3"]
}

fn wrapped_9() -> Wrapper<[u8; 9]> {
    Wrapper([9,8,7,6,5,4,3,2,1])
}

fn wrapped_8() -> Wrapper<[u8; 8]> {
    Wrapper([8,7,6,5,4,3,2,1])
}

fn multi_9() -> [[u8; 3]; 3] {
    [[1,2,3],[4,5,6],[7,8,9]]
}

fn multi_4x2() -> [[u8; 4]; 2] {
    [[1,2,3,4],[5,6,7,8]]
}

fn multi_2x4() -> [[u8; 2]; 4] {
    [[1,2],[3,4],[5,6],[7,8]]
}

fn multi_w_9() -> Wrapper<[[u8; 3]; 3]> {
    Wrapper([[9,8,7],[6,5,4],[3,2,1]])
}

fn multi_w_4x2() -> Wrapper<[[u8; 4]; 2]> {
    Wrapper([[8,7,6,5],[4,3,2,1]])
}

fn multi_w_2x4() -> Wrapper<[[u8; 2]; 4]> {
    Wrapper([[8,7],[6,5],[4,3],[2,1]])
}

fn main() {
    u8_10(); // 10 bytes; pass by stack
    u8_9(); // 9 bytes; pass by stack
    u8_8(); // 8 bytes; pass by register
    u8_6(); // 6 bytes; pass by register
    u8_4(); // 4 bytes; pass by register
    u8_2(); // 2 bytes; pass by register
    u8_1(); // 1 bytes; pass by register

    u16_1(); // 2 bytes; pass by register
    u16_2(); // 4 bytes; pass by register
    u16_4(); // 8 bytes; pass by register
    u16_5(); // 10 bytes; pass by stack
    u16_6(); // 12 bytes; pass by stack

    i16_1(); // 2 bytes; pass by register
    i16_2(); // 4 bytes; pass by register
    i16_4(); // 8 bytes; pass by register
    i16_5(); // 10 bytes; pass by stack
    i16_6(); // 12 bytes; pass by stack

    u32_1(); // 4 bytes; pass by register
    u32_2(); // 8 bytes; pass by register
    u32_3(); // 12 bytes; pass by stack
    u32_4(); // 16 bytes; pass by stack

    i32_1(); // 4 bytes; pass by register
    i32_2(); // 8 bytes; pass by register
    i32_3(); // 12 bytes; pass by stack
    i32_4(); // 16 bytes; pass by stack

    u64_1(); // 8 bytes; pass by register
    u64_2(); // 16 bytes; pass by stack
    u64_3(); // 24 bytes; pass by stack

    i64_1(); // 8 bytes; pass by register
    i64_2(); // 16 bytes; pass by stack
    i64_3(); // 24 bytes; pass by stack

    u128_1(); // 16 bytes; pass by stack
    u128_2(); // 32 bytes; pass by stack
    u128_3(); // 48 bytes; pass by stack

    i128_1(); // 16 bytes; pass by stack
    i128_2(); // 32 bytes; pass by stack
    i128_3(); // 48 bytes; pass by stack

    f32_1(); // 4 bytes; pass by register
    f32_2(); // 8 bytes; pass by register
    f32_3(); // 12 bytes; pass by stack

    f64_1(); // 8 bytes; pass by register
    f64_2(); // 16 bytes; pass by stack
    f64_3(); // 24 bytes; pass by stack

    usize_1(); // 8 bytes; pass by register
    usize_2(); // 16 bytes; pass by stack
    usize_3(); // 24 bytes; pass by stack

    isize_1(); // 8 bytes; pass by register
    isize_2(); // 16 bytes; pass by stack
    isize_3(); // 24 bytes; pass by stack

    str_1(); // 16 bytes; pass by stack
    str_2(); // 32 bytes; pass by stack
    str_3(); // 48 bytes; pass by stack

    wrapped_9(); // 9 bytes; pass by stack
    wrapped_8(); // 8 bytes; pass by register

    multi_9(); // 9 bytes; pass by stack
    multi_4x2(); // 8 bytes; pass by register
    multi_2x4(); // 8 bytes; pass by register

    multi_w_9(); // 9 bytes; pass by stack
    multi_w_4x2(); // 8 bytes; pass by register
    multi_w_2x4(); // 8 bytes; pass by register
}