#[inline(never)]
pub fn multi_return(i: i32) -> i32 {
    if i == 1 {
        if std::hint::black_box(true) {
            return 11;
        } else {
            return 1;
        }
    }
    if i == 2 {
        if std::hint::black_box(true) {
            return std::hint::black_box(11) * std::hint::black_box(2);
        } else {
            return 2;
        }
    }
    if i == 3 {
        if std::hint::black_box(true) {
            return std::hint::black_box(30) + 3;
        } else {
            return 3;
        }
    }
    0
}

fn main() {
    assert_eq!(multi_return(std::hint::black_box(1)), 11);
    assert_eq!(multi_return(std::hint::black_box(2)), 22);
    assert_eq!(multi_return(std::hint::black_box(3)), 33);
    assert_eq!(multi_return(std::hint::black_box(4)), 0);
}
