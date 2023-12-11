fn u8(v: u8) -> u8 { println!("\nu8\t{v}"); println!("u8\t{:#0x}", v); v }
fn i8(v: i8) -> i8 { println!("\ni8\t{v}"); println!("i8\t{:#0x}", v); v }
fn u16(v: u16) -> u16 { println!("\nu16\t{v}"); println!("u16\t{:#0x}", v); v }
fn i16(v: i16) -> i16 { println!("\ni16\t{v}"); println!("i16\t{:#0x}", v); v }
fn u32(v: u32) -> u32 { println!("\nu32\t{v}"); println!("u32\t{:#0x}", v); v }
fn i32(v: i32) -> i32 { println!("\ni32\t{v}"); println!("i32\t{:#0x}", v); v }
fn u64(v: u64) -> u64 { println!("\nu64\t{v}"); println!("u64\t{:#0x}", v); v }
fn i64(v: i64) -> i64 { println!("\ni64\t{v}"); println!("i64\t{:#0x}", v); v }
fn u128(v: u128) -> u128 { println!("\nu128\t{v}"); println!("u128\t{:#0x}", v); v }
fn i128(v: i128) -> i128 { println!("\ni128\t{v}"); println!("i128\t{:#0x}", v); v }
fn usize(v: usize) -> usize { println!("\nusize\t{v}"); println!("usize\t{:#0x}", v); v }
fn isize(v: isize) -> isize { println!("\nisize\t{v}"); println!("isize\t{:#0x}", v); v }
fn f32(v: f32) -> f32 { println!("\nf32\t{v}"); v }
fn f64(v: f64) -> f64 { println!("\nf64\t{v}"); v }
fn bool(v: bool) -> bool { println!("\nbool\t{v}"); v }
fn static_str(v: &'static str) -> &'static str { println!("\n&str\t{v}"); "hello" }
fn result_ok(v: i32) -> Result<i64, i64> { println!("\nok\t{v}"); Ok(0x1234) }
fn result_err(v: i32) -> Result<i32, i32> { println!("\nerr\t{v}"); Err(0x5678) }
fn result_ok_u32(v: i32) -> Result<u32, ()> { println!("\nok\t{v}"); Ok(1234) }
fn result_err_u64(v: i32) -> Result<(), u64> { println!("\nerr\t{v}"); Err(12345678) }

fn main() {
    u8(u8::MAX);
    u8(u8::MIN);
    u8(u8::from_str_radix("fa", 16).unwrap());

    i8(i8::MAX);
    i8(i8::MIN);
    i8(i8::from_str_radix("76", 16).unwrap());

    u16(u16::MAX);
    u16(u16::MIN);
    u16(u16::from_str_radix("fafa", 16).unwrap());

    i16(i16::MAX);
    i16(i16::MIN);
    i16(i16::from_str_radix("7654", 16).unwrap());

    u32(u32::MAX);
    u32(u32::MIN);
    u32(u32::from_str_radix("fafafafa", 16).unwrap());

    i32(i32::MAX);
    i32(i32::MIN);
    i32(i32::from_str_radix("76543210", 16).unwrap());

    u64(u64::MAX);
    u64(u64::MIN);
    u64(u64::from_str_radix("9876543210abcdef", 16).unwrap());

    i64(i64::MAX);
    i64(i64::MIN);
    i64(i64::from_str_radix("9876543210abcde", 16).unwrap());

    u128(u128::MAX);
    u128(u128::MIN);
    u128(u128::from_str_radix("9876543210abcdeffedcba0123456789", 16).unwrap());
    u128(u128::from_str_radix("0000000000000000fedcba0123456789", 16).unwrap());
    u128(u128::from_str_radix("fedcba01234567890000000000000000", 16).unwrap());

    i128(i128::MAX);
    i128(i128::MIN);
    i128(i128::from_str_radix("9876543210abcdeffedcba012345678", 16).unwrap());
    i128(i128::from_str_radix("0000000000000000fedcba012345678", 16).unwrap());
    i128(i128::from_str_radix("fedcba0123456789000000000000000", 16).unwrap());

    usize(usize::MAX);
    usize(usize::MIN);
    usize(usize::from_str_radix("9876543210abcdef", 16).unwrap());

    isize(isize::MAX);
    isize(isize::MIN);
    isize(isize::from_str_radix("9876543210abcde", 16).unwrap());

    f32(f32::MAX);
    f32(f32::MIN);
    f32(3.14159265358979323846264338327950288);

    f64(f64::MAX);
    f64(f64::MIN);
    f64(3.14159265358979323846264338327950288);

    bool(true);
    bool(false);

    static_str("hi");

    result_ok(1);
    result_err(2);
    result_ok_u32(3);
    result_err_u64(4);
}
