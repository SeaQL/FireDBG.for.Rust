struct Train<'a, 'b> {
    head: &'a Head<'a>,
    cargo: Vec<Cargo>,
    tail: &'b Tail<'b>,
}

struct Head<'a>(&'a Label<'a>);

struct Tail<'a> {
    label: &'a Label<'a>,
    end: i32,
}

struct Cargo {
    payload: u8,
}

#[derive(Copy, Clone)]
struct Label<'a>(&'a str);

fn depart<'a, 'b, 'c>(train: &'c Train<'a, 'b>) {
    driver(train.head, train.tail);
}

fn driver(head: &Head, tail: &Tail) {
    println!("head = Head(Label(\"{}\"))", head.0 .0);
    println!(
        "tail = Tail {{ label: Label(\"{}\"), end: {} }}",
        tail.label.0, tail.end
    );
}

fn main() {
    let label = Label("Bullet");
    let head = Head(&label);
    let tail = Tail {
        label: &label,
        end: 88888,
    };
    let train = Train {
        head: &head,
        cargo: vec![
            Cargo { payload: 1 },
            Cargo { payload: 2 },
            Cargo { payload: 3 },
            Cargo { payload: 4 },
        ],
        tail: &tail,
    };
    depart(&train);
}
