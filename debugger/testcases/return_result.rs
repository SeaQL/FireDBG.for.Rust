use std::{rc::Rc, fmt::Debug};

fn res_1(i: i32) -> Result<(), ()> {
    if i == 0 { Ok(()) } else { Err(()) }
}
fn res_2(i: i32) -> Result<i8, ()> {
    if i == 0 { Ok(8) } else { Err(()) }
}
fn res_3(i: i32) -> Result<i32, i32> {
    if i == 0 { Ok(88888) } else { Err(-222_222) }
}
fn res_4(i: i32) -> Result<u32, i32> {
    if i == 0 { Ok(88888) } else { Err(-222_222) }
}
fn res_5(i: i32) -> Result<i8, u8> {
    if i == 0 { Ok(-2) } else { Err(250) }
}
fn res_6(i: i32) -> Result<i16, i32> {
    if i == 0 { Ok(-4444) } else { Err(222_222) }
}
fn res_7(i: i32) -> Result<i32, i64> {
    if i == 0 { Ok(-222_222) } else { Err(22_222_222_222) }
}
fn res_8(i: i32) -> Result<i8, i8> {
    if i == 0 { Ok(22) } else { Err(-108) }
}
fn res_9(i: i32) -> Result<u64, i64> {
    if i == 0 { Ok(22_222_222_222) } else { Err(-222_222_222) }
}
fn res_10(i: i32) -> Result<f32, f32> {
    if i == 0 { Ok(2.2) } else { Err(3.3) }
}
fn res_11(i: i32) -> Result<f32, f64> {
    if i == 0 { Ok(2.2) } else { Err(3.3) }
}
fn res_12(i: i32) -> Result<f64, f64> {
    if i == 0 { Ok(2.2) } else { Err(3.3) }
}
fn result_ref<'a>(a: &'a i32, b: &'a u64) -> Result<&'a i32, &'a u64> {
    if *a > 10 { Ok(a) } else { Err(b) }
}
fn res_small(i: i32) -> Result<Small, i32> {
    if i == 0 { Ok(Small{ i: Inner(8888) }) } else { Err(-222_222) }
}
fn res_big(i: i32) -> Result<Small, Big> {
    if i == 0 { Ok(Small{ i: Inner(8888) }) } else { Err(Big{ i: Inner(2222), t: -101 }) }
}
fn res_str_1(i: i32) -> Result<(), &'static str> {
    if i == 0 { Ok(()) } else { Err("hello") }
}
fn res_str_2(i: i32) -> Result<&'static str, ()> {
    if i == 0 { Ok("world") } else { Err(()) }
}
fn res_13(i: i32) -> Result<(), bool> {
    match i {
        0 => Ok(()),
        1 => Err(true),
        2 => Err(false),
        _ => panic!("Unexpected i {i}")
    }
}
fn res_14(i: i32) -> Result<bool, ()> {
    match i {
        0 => Err(()),
        1 => Ok(true),
        2 => Ok(false),
        _ => panic!("Unexpected i {i}")
    }
}
fn res_15(i: i32) -> Result<bool, bool> {
    match i {
        0 => Ok(false),
        1 => Ok(true),
        2 => Err(false),
        3 => Err(true),
        _ => panic!("Unexpected i {i}")
    }
}
fn res_16(i: i32) -> Result<i128, i128> {
    if i == 0 { Ok(22_222_222_222_222_222_222) } else { Err(-22_222_222_222_222_222_222) }
}
fn res_17(i: i32) -> Result<(), i128> {
    if i == 0 { Ok(()) } else { Err(-22_222_222_222_222_222_222) }
}
fn res_18(i: i32) -> Result<u128, ()> {
    if i == 0 { Ok(22_222_222_222_222_222_222) } else { Err(()) }
}
fn res_19(i: i32) -> Result<i128, u128> {
    if i == 0 { Ok(-22_222_222_222_222_222_222) } else { Err(170141183460469231731687303715884105727) }
}
fn res_20(i: i32) -> Result<Box<dyn Mass>, ()> {
    if i == 0 { Ok(Box::new(Small{ i: Inner(8888) })) } else { Err(()) }
}
fn res_21(i: i32) -> Result<Box<dyn Mass>, Box<dyn Mass>> {
    if i == 0 { Ok(Box::new(Big{ i: Inner(2222), t: -101 })) } else { Err(Box::new(Small{ i: Inner(8888) })) }
}
fn res_22(i: i32) -> Result<Rc<dyn Mass>, ()> {
    if i == 0 { Ok(Rc::new(Small{ i: Inner(8888) })) } else { Err(()) }
}
fn res_23(i: i32) -> Result<Rc<dyn Mass>, Rc<dyn Mass>> {
    if i == 0 { Ok(Rc::new(Big{ i: Inner(2222), t: -101 })) } else { Err(Rc::new(Small{ i: Inner(8888) })) }
}
fn res_24(i: i32) -> Result<char, ()> {
    if i == 0 { Ok('🔥') } else { Err(()) }
}
fn res_25(i: i32) -> Result<(), char> {
    if i == 0 { Ok(()) } else { Err('🔥') }
}
fn res_26(i: i32) -> Result<&'static [u8], ()> {
    if i == 0 { Ok(&[1,2,3]) } else { Err(()) }
}
fn res_27(i: i32) -> Result<(), &'static [char]> {
    if i == 0 { Ok(()) } else { Err(&['🌊','🦦','🦀']) }
}

#[derive(Debug)]
struct Small {
    i: Inner,
}
#[derive(Debug)]
struct Big {
    i: Inner,
    t: i64,
}
#[derive(Debug)]
struct Inner(i32);

trait Mass: Debug {}
impl Mass for Big {}
impl Mass for Small {}

fn main() {
    let res = res_1(0); dbg!(&res);
    let res = res_1(2); dbg!(&res);
    let res = res_2(0); dbg!(&res);
    let res = res_2(2); dbg!(&res);
    let res = res_3(0); dbg!(&res);
    let res = res_3(2); dbg!(&res);
    let res = res_4(0); dbg!(&res);
    let res = res_4(2); dbg!(&res);
    let res = res_5(0); dbg!(&res);
    let res = res_5(2); dbg!(&res);
    let res = res_6(0); dbg!(&res);
    let res = res_6(2);
    match res {
        Ok(r) => println!("Ok({r})"),
        Err(e) => println!("Ok({e})"),
    }
    let res = res_7(0); dbg!(&res);
    let res = res_7(2); dbg!(&res);
    let res = res_8(0); dbg!(&res);
    let res = res_8(2); dbg!(&res);
    let res = res_9(0); dbg!(&res);
    let res = res_9(2); dbg!(&res);
    let res = res_10(0); dbg!(&res);
    let res = res_10(2); dbg!(&res);
    let res = res_11(0); dbg!(&res);
    let res = res_11(2); dbg!(&res);
    let res = res_12(0); dbg!(&res);
    let res = res_12(2); dbg!(&res);
    let a = 8; let b = 12; let c = result_ref(&a, &b); dbg!(&c);
    let a = 2222; let b = 24; let c = result_ref(&a, &b); dbg!(&c);
    let res = res_small(0); dbg!(&res); // through register
    let res = res_small(2); dbg!(&res); // through register
    let res = res_big(0); dbg!(&res); // through stack
    let res = res_big(2); dbg!(&res); // through stack
    let res = res_str_1(0); dbg!(&res);
    let res = res_str_1(2); dbg!(&res);
    let res = res_str_2(0); dbg!(&res);
    let res = res_str_2(2); dbg!(&res);
    let res = res_13(0); dbg!(&res);
    let res = res_13(1); dbg!(&res);
    let res = res_13(2); dbg!(&res);
    let res = res_14(0); dbg!(&res);
    let res = res_14(1); dbg!(&res);
    let res = res_14(2); dbg!(&res);
    let res = res_15(0); dbg!(&res);
    let res = res_15(1); dbg!(&res);
    let res = res_15(2); dbg!(&res);
    let res = res_15(3); dbg!(&res);
    let res = res_16(0); dbg!(&res);
    let res = res_16(2); dbg!(&res);
    let res = res_17(0); dbg!(&res);
    let res = res_17(2); dbg!(&res);
    let res = res_18(0); dbg!(&res);
    let res = res_18(2); dbg!(&res);
    let res = res_19(0); dbg!(&res);
    let res = res_19(2); dbg!(&res);
    let res = res_20(0); dbg!(&res);
    let res = res_20(2); dbg!(&res);
    let res = res_21(0); dbg!(&res);
    let res = res_21(2); dbg!(&res);
    let res = res_22(0); dbg!(&res);
    let res = res_22(2); dbg!(&res);
    let res = res_23(0); dbg!(&res);
    let res = res_23(2); dbg!(&res);
    let res = res_24(0); dbg!(&res);
    let res = res_24(2); dbg!(&res);
    let res = res_25(0); dbg!(&res);
    let res = res_25(2); dbg!(&res);
    let res = res_26(0); dbg!(&res);
    let res = res_26(2); dbg!(&res);
    let res = res_27(0); dbg!(&res);
    let res = res_27(2); dbg!(&res);
}
