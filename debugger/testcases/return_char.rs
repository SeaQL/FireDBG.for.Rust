const X: char = 'X';
const O: char = 'O';

struct Alpha {
    c1: char,
    c2: char,
}
fn alpha() -> Alpha {
    Alpha {
        c1: X,
        c2: O,
    }
}

struct Beta {
    arr: [i32; 3],
    c: char,
}
fn beta(arr: [i32; 3]) -> Beta {
    Beta {
        arr,
        c: O,
    }
}

struct Charlie {
    arr: [[char; 3]; 3],
    c: char,
}
fn charlie(arr: [[char; 3]; 3]) -> Charlie {
    Charlie {
        arr,
        c: X,
    }
}

fn delta(a: bool, c: char) -> Option<char> {
    if a {
        Some(c)
    } else {
        None
    }
}

fn main() {
    let alpha = alpha();
    let beta = beta([1, 2, 3]);
    let charlie = charlie([
        [X, O, X],
        [O, X, O],
        [X, X, X],
    ]);
    let delta_some = delta(true, O);
    let delta_none = delta(false, X);
    let delta_min = delta(true, '\0');
    let delta_null = delta(true, char::REPLACEMENT_CHARACTER);
    let delta_max = delta(true, char::MAX);
}
