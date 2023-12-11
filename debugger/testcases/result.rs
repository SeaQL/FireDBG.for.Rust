#[derive(Debug)]
struct Good(i32);

#[derive(Debug)]
struct Bad {
    i: u32,
}

fn func(v: Result<Good, Bad>) -> Result<Good, Bad> {
    match v {
        Ok(Good(v)) => Err(Bad { i: (v + 1) as u32 }),
        Err(Bad { i }) => Ok(Good((i + 1) as i32)),
    }
}

fn main() {
    let ok = func(Err(Bad { i: 1234 }));
    dbg!(ok);
    let err = func(Ok(Good(5678)));
    dbg!(err);
    println!();
}
