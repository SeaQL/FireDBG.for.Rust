use std::fmt::Debug;

fn res_0(i: i32) -> Option<()> {
    if i > 0 { Some(()) } else { None }
}
fn res_1(i: i32) -> Option<bool> {
    if i > 0 { Some(true) } else { None }
}
fn res_2(i: i32) -> Option<i8> {
    if i > 0 { Some(-22) } else { None }
}
fn res_3(i: i32) -> Option<u8> {
    if i > 0 { Some(250) } else { None }
}
fn res_4(i: i32) -> Option<i16> {
    if i > 0 { Some(-22222) } else { None }
}
fn res_5(i: i32) -> Option<u16> {
    if i > 0 { Some(65432) } else { None }
}
fn res_6(i: i32) -> Option<i32> {
    if i > 0 { Some(-222_222) } else { None }
}
fn res_7(i: i32) -> Option<u32> {
    if i > 0 { Some(432_432) } else { None }
}
fn res_8(i: i32) -> Option<i64> {
    if i > 0 { Some(-22_222_222_222) } else { None }
}
fn res_9(i: i32) -> Option<u64> {
    if i > 0 { Some(23_232_232_232) } else { None }
}
fn res_10(i: i32) -> Option<isize> {
    if i > 0 { Some(-22_222_222_222) } else { None }
}
fn res_11(i: i32) -> Option<usize> {
    if i > 0 { Some(23_232_232_232) } else { None }
}
fn res_12(i: i32) -> Option<i128> {
    if i > 0 { Some(-22_222_222_222_222_222_222) } else { None }
}
fn res_13(i: i32) -> Option<u128> {
    if i > 0 { Some(33_333_333_333_333_333_333) } else { None }
}
fn res_14(i: i32) -> Option<f32> {
    if i > 0 { Some(111.111) } else { None }
}
fn res_15(i: i32) -> Option<f64> {
    if i > 0 { Some(222.222) } else { None }
}
fn res_16(i: i32) -> Option<&'static str> {
    if i > 0 { Some("hello") } else { None }
}
fn res_17(i: i32) -> Option<char> {
    if i > 0 { Some(0x40 as char) } else { None }
}
fn res_18(i: i32) -> Option<&'static [u8]> {
    if i > 0 { Some(&[1,2,3]) } else { None }
}
fn res_19(i: i32) -> Option<&'static [char]> {
    if i > 0 { Some(&['🌊','🦦','🦀']) } else { None }
}
fn res_obj(i: i32) -> Option<Car> {
    if i > 0 {
        Some(Car {
            brand: "Ford",
            engine: Engine {
                config: EngineConfig::Inline { i: 4 },
                pistons: vec![Piston(1), Piston(2), Piston(3), Piston(4)],
            },
            gearbox: Gearbox::Manual,
        })
    } else {
        None
    }
}
fn res_obj_ref<'a>(cars: &[Car], i: usize) -> Option<&Car> {
    Some(&cars[i])
}
fn res_small(i: i32) -> Option<Small> {
    if i > 0 { Some(Small { i: Inner(12345678) }) } else { None }
}
fn res_box(i: i32) -> Option<Box<dyn Auto>> {
    if i > 0 {
        Some(Box::new(Car {
            brand: "Nil",
            engine: Engine {
                config: EngineConfig::Inline { i: 0 },
                pistons: vec![],
            },
            gearbox: Gearbox::Automatic,
        }))
    } else {
        None
    }
}

fn res_enum(i: i32) -> Option<Gearbox> {
    if i == 1 {
        Some(Gearbox::Automatic)
    } else if i == 2 {
        Some(Gearbox::Manual)
    } else {
        None
    }
}

#[derive(Debug)]
struct Car {
    brand: &'static str,
    engine: Engine,
    gearbox: Gearbox,
}

#[derive(Debug)]
struct Engine {
    config: EngineConfig,
    pistons: Vec<Piston>,
}

#[derive(Debug)]
struct Piston(u8);

#[derive(Debug)]
enum Gearbox {
    Automatic,
    Manual,
}

#[derive(Debug)]
enum EngineConfig {
    Inline { i: i32 },
    Vshape(i16, i16),
}

#[derive(Debug)]
struct Small {
    i: Inner,
}
#[derive(Debug)]
struct Inner(i32);

trait Auto: Debug {}
impl Auto for Car {}

fn main() {
    let res = res_0(0); dbg!(&res);
    let res = res_0(2); dbg!(&res);
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
    let res = res_6(2); dbg!(&res);
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
    let res = res_13(0); dbg!(&res);
    let res = res_13(2); dbg!(&res);
    let res = res_14(0); dbg!(&res);
    let res = res_14(2); dbg!(&res);
    let res = res_15(0); dbg!(&res);
    let res = res_15(2); dbg!(&res);
    let res = res_16(0); dbg!(&res);
    let res = res_16(2); dbg!(&res);
    let res = res_17(0); dbg!(&res);
    let res = res_17(2); dbg!(&res);
    let res = res_18(0); dbg!(&res);
    let res = res_18(2); dbg!(&res);
    let res = res_19(0); dbg!(&res);
    let res = res_19(2); dbg!(&res);
    let none = res_obj(0); dbg!(&none);
    let ford = res_obj(2); dbg!(&ford);
    let ford = ford.unwrap();
    let mazda = Car {
        brand: "Mazda",
        engine: Engine {
            config: EngineConfig::Vshape(3, 3),
            pistons: vec![],
        },
        gearbox: Gearbox::Automatic,
    };
    let cars = [ford, mazda];
    let car = res_obj_ref(&cars, 1); dbg!(car);
    let res = res_small(0); dbg!(&res);
    let res = res_small(2); dbg!(&res);
    let res = res_box(0); dbg!(&res);
    let res = res_box(2); dbg!(&res);
    let res = res_enum(0); dbg!(&res);
    let res = res_enum(1); dbg!(&res);
    let res = res_enum(2); dbg!(&res);
}
