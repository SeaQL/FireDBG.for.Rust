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

fn create_manual_car() -> Car {
    Car {
        brand: "Ford",
        engine: Engine {
            config: EngineConfig::Inline { i: 4 },
            pistons: vec![Piston(1), Piston(2), Piston(3), Piston(4)],
        },
        gearbox: Gearbox::Manual,
    }
}

fn create_auto_car() -> Car {
    let car = Car {
        brand: "Mazda",
        engine: Engine {
            config: EngineConfig::Vshape(3, 3),
            pistons: vec![],
        },
        gearbox: Gearbox::Automatic,
    };
    car
}

fn choose_a_car_for_me<'a>(first: bool, a: &'a Car, b: &'a Car) -> &'a Car {
    if std::hint::black_box(first) {
        a
    } else {
        b
    }
}

fn main() {
    let man = create_manual_car();
    println!("{man:?}");

    let car = Car {
        brand: "Nil",
        engine: Engine {
            config: EngineConfig::Inline { i: 0 },
            pistons: vec![],
        },
        gearbox: Gearbox::Automatic,
    };

    let auto = create_auto_car();
    println!("{auto:?}");

    let what_car = choose_a_car_for_me(true, &auto, &man);
    println!("{what_car:?}");

    let what_car_again = choose_a_car_for_me(false, &auto, &man);
    println!("{what_car_again:?}");
}
