use std::sync::{Arc, Weak};

#[derive(Debug)]
struct Cart {
    name: String,
    next: StrongOrWeak<Cart>,
}

#[derive(Debug)]
enum StrongOrWeak<T> {
    Strong(Arc<T>),
    Weak(Weak<T>),
}

fn main() {
    let head = Arc::new_cyclic(|head| Cart {
        name: "head".to_owned(),
        next: StrongOrWeak::Strong(Arc::new(Cart {
            name: "tail".to_owned(),
            next: StrongOrWeak::Weak(head.clone()),
        })),
    });
    dbg!(head);
}
