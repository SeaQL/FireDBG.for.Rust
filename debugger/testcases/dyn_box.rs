use std::{sync::Arc, rc::Rc};

trait MyTrait {
    fn i(&self) -> i32;
}

struct MyStruct {
    i: i32,
}

struct MyOther {
    not_i: i64,
}

impl MyTrait for MyStruct {
    fn i(&self) -> i32 {
        self.i
    }
}

impl MyTrait for MyOther {
    fn i(&self) -> i32 {
        self.not_i.try_into().unwrap()
    }
}

fn open<T>(boxed: T) {
    std::hint::black_box(boxed);
}

fn main() {
    let boxed: Box<dyn MyTrait> = Box::new(MyStruct { i: 1234 });
    println!("Box {{ i: {} }}", boxed.i());
    open(&boxed);
    let rc: Rc<dyn MyTrait> = Rc::new(MyStruct { i: 1234 });
    println!("Rc {{ i: {} }}", rc.i());
    open(&rc);
    let arc: Arc<dyn MyTrait> = Arc::new(MyOther { not_i: 5678 });
    println!("Arc {{ i: {} }}", arc.i());
    open(&arc);
}