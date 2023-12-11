#[derive(Debug, Copy, Clone)]
enum Size {
    Small,
    Medium,
    Big,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
#[repr(u16)]
enum Greek {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
}

fn shrink(size: Size) -> Size {
    match size {
        Size::Small => panic!("Can't be smaller"),
        Size::Medium => Size::Small,
        Size::Big => Size::Medium,
    }
}

fn flip(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

fn advance(greek: Greek) -> Greek {
    match greek {
        Greek::Alpha => Greek::Beta,
        Greek::Beta => Greek::Gamma,
        Greek::Gamma => Greek::Delta,
        Greek::Delta => Greek::Epsilon,
        Greek::Epsilon => panic!("No where"),
    }
}

fn main() {
    let big = Size::Big;
    let medium = shrink(big);
    assert!(matches!(medium, Size::Medium));
    dbg!((big, medium));

    let north = Direction::North;
    let south = flip(north);
    assert!(matches!(south, Direction::South));
    dbg!((north, south));

    let east = Direction::East;
    let west = flip(east);
    assert!(matches!(west, Direction::West));
    dbg!((east, west));

    let alpha = Greek::Alpha;
    let beta = advance(alpha);
    assert!(matches!(beta, Greek::Beta));
    dbg!((alpha, beta));

    let beta = Greek::Beta;
    let gamma = advance(beta);
    assert!(matches!(gamma, Greek::Gamma));
    dbg!((beta, gamma));

    let gamma = Greek::Gamma;
    let delta = advance(gamma);
    assert!(matches!(delta, Greek::Delta));
    dbg!((gamma, delta));
}
