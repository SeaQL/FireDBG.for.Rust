#[derive(Debug)]
struct Small {
    a: i32,
    b: i64,
}

#[derive(Debug)]
struct Big {
    a: i32,
    b: i64,
    c: &'static str,
}

fn return_ref_bool<'a>(a: &'a bool, b: &'a bool) -> &'a bool {
    if std::hint::black_box(true) { a } else { b }
}
fn return_ref_u8<'a>(a: &'a u8, b: &'a u8) -> &'a u8 {
    if std::hint::black_box(true) { a } else { b }
}
fn return_ref_i32<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if std::hint::black_box(true) { a } else { b }
}
fn return_ref_i64<'a>(a: &'a i64, b: &'a i64) -> &'a i64 {
    if std::hint::black_box(true) { a } else { b }
}
fn return_ref_i128<'a>(a: &'a i128, b: &'a i128) -> &'a i128 {
    if std::hint::black_box(true) { a } else { b }
}
fn return_ref_f32<'a>(a: &'a f32, b: &'a f32) -> &'a f32 {
    if std::hint::black_box(true) { a } else { b }
}
fn return_ref_f64<'a>(a: &'a f64, b: &'a f64) -> &'a f64 {
    if std::hint::black_box(true) { a } else { b }
}

fn return_ref_small<'a>(a: &'a Small, b: &'a Small) -> &'a Small {
    if std::hint::black_box(true) { a } else { b }
}

fn return_ref_big<'a>(a: &'a Big, b: &'a Big) -> &'a Big {
    if std::hint::black_box(false) { a } else { b }
}

fn main() {
    let a = true; let b = false; let c = return_ref_bool(&a, &b); dbg!(c);
    let a = 22; let b = 33; let c = return_ref_u8(&a, &b); dbg!(c);
    let a = 222_222; let b = 333_333; let c = return_ref_i32(&a, &b); dbg!(c);
    let a = 22_222_222_222; let b = 33_333_333_333; let c = return_ref_i64(&a, &b); dbg!(c);
    let a = 22_222_222_222_222_222_222; let b = 33_333_333_333_333_333_333; let c = return_ref_i128(&a, &b); dbg!(c);
    let a = 2.0; let b = 3.0; let c = return_ref_f32(&a, &b); dbg!(c);
    let a = 2.0; let b = 3.0; let c = return_ref_f64(&a, &b); dbg!(c);
    let a = Small { a: 2, b: 3 }; let b = Small { a: 3, b: 2 }; let c = return_ref_small(&a, &b); dbg!(c);
    let a = Big { a: 2, b: 3, c: "4" }; let b = Big { a: 4, b: 3, c: "2" }; let c = return_ref_big(&a, &b); dbg!(c);
}
