use std::{sync::Arc, rc::Rc, fmt::Debug};

#[derive(Debug)]
struct Small {
    a: i32,
    b: i64,
}

#[derive(Debug)]
struct Big {
    a: i32,
    b: i64,
    c: &'static str,
}

trait Mass: Debug {}
impl Mass for Big {}

fn return_boxed_big() -> Box<Big> {
    Box::new(Big { a: 1, b: 2, c: "box" })
}

fn return_rc_big() -> Rc<Big> {
    Rc::new(Big { a: 2, b: 3, c: "rc" })
}

fn return_arc_big() -> Arc<Big> {
    Arc::new(Big { a: 3, b: 4, c: "arc" })
}

fn return_dyn_box_big() -> Box<dyn Mass> {
    Box::new(Big { a: 0, b: 0, c: "0" })
}

fn return_boxed_slice() -> Box<&'static [u8]> {
    Box::new(&[1,2,3])
}

fn return_rc_slice() -> Rc<&'static [u8]> {
    Rc::new(&[1,2,3])
}

fn return_arc_slice() -> Arc<&'static [u8]> {
    Arc::new(&[1,2,3])
}

fn main() {
    let r = return_boxed_big(); dbg!(&r);
    let r = return_rc_big(); dbg!(&r);
    let r = return_arc_big(); dbg!(&r);
    let r = return_dyn_box_big(); dbg!(&r);
    let r = return_boxed_slice(); dbg!(&r);
    let r = return_rc_slice(); dbg!(&r);
    let r = return_arc_slice(); dbg!(&r);
    println!();
}
