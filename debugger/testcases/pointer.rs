use std::{rc::Rc, fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
struct Object {
    name: String,
}
trait Shape: Debug {}
impl Shape for Object {}

fn main() {
    {
        let boxed = Box::new(Object {
            name: "Boxed".to_owned(),
        });
        let arc = Arc::new(Object {
            name: "Arced".to_owned(),
        });
        let rc = Rc::new(Object {
            name: "Rced".to_owned(),
        });
        let _1 = arc.clone();
        let _2 = rc.clone();
        let _3 = Arc::downgrade(&arc);
        let _4 = Arc::downgrade(&arc);
        let _5 = rc.clone();
        let _6 = Rc::downgrade(&rc);
        dbg!(&boxed);
        dbg!(&arc);
        dbg!(&rc);
        println!();
    }
    {
        let obj = Object {
            name: "Don't care".to_owned(),
        };
        let boxed: Box<dyn Shape> = Box::new(obj.clone());
        let arc: Arc<dyn Shape> = Arc::new(obj.clone());
        let rc: Rc<dyn Shape> = Rc::new(obj.clone());
        let obj: &dyn Shape = &obj;
        dbg!(&boxed);
        dbg!(&arc);
        dbg!(&rc);
        dbg!(&obj);
        println!();
    }
}
