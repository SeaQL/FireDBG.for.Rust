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

fn main() {
    let man = Car {
        brand: "Ford",
        engine: Engine {
            config: EngineConfig::Inline { i: 4 },
            pistons: vec![Piston(1), Piston(2), Piston(3), Piston(4)],
        },
        gearbox: Gearbox::Manual,
    };

    println!("{man:?}");

    let auto = Box::new(Car {
        brand: "Mazda",
        engine: Engine {
            config: EngineConfig::Vshape(3, 3),
            pistons: vec![],
        },
        gearbox: Gearbox::Automatic,
    });

    println!("{auto:?}");
}
