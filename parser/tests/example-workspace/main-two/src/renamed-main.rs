fn head(i: i32) -> i32 {
    inter(i + 1)
}

fn inter(i: i32) -> i32 {
    tail(i)
}

fn tail(i: i32) -> i32 {
    end(i) + 1
}

fn end(i: i32) -> i32 {
    i
}

fn renamed_main() -> i32 {
    1
}

fn main() {
    let res = head(1);
    assert_eq!(res, 3);
    dbg!(res);
    renamed_main();
}
