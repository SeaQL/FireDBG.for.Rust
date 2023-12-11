use std::fmt::{Display, Formatter};

#[derive(Clone)]
struct World {
    nth: i32,
}

fn hello_1(world: World) {
    println!("hello {}", world);
}

fn hello_2(world: &World) {
    println!("hello {}", world);
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}th world", self.nth)
    }
}

fn main() {
    let world = World { nth: 99 };
    hello_1(world.clone());
    hello_2(&world);
}
