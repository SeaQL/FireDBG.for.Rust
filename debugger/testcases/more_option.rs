use std::{rc::Rc, sync::Arc};

struct BNode {
    i: i32,
    next: Option<Box<BNode>>,
}
struct RCNode {
    i: i32,
    next: Option<Rc<RCNode>>,
}
struct RCCNode {
    i: i32,
    next: Option<Arc<RCCNode>>,
}

fn capture<T>(v: &T) {
    std::hint::black_box(v);
}

fn main() {
    let node = BNode {
        i: 1,
        next: Some(Box::new(BNode {
            i: 2,
            next: None,
        }))
    };
    capture(&node);

    let node = BNode {
        i: 1,
        next: Some(Box::new(BNode {
            i: 2,
            next: Some(Box::new(BNode {
                i: 3,
                next: None,
            })),
        }))
    };
    capture(&node);

    let dyn_box: Box<dyn std::fmt::Debug> = Box::new("hello");
    capture(&dyn_box);
    capture(&Some(dyn_box));

    let node = RCNode {
        i: 11,
        next: None,
    };
    capture(&node);

    let node = RCNode {
        i: 11,
        next: Some(Rc::new(RCNode {
            i: 22,
            next: None,
        })),
    };
    capture(&node);

    let node = RCNode {
        i: 11,
        next: Some(Rc::new(RCNode {
            i: 22,
            next: Some(Rc::new(RCNode {
                i: 33,
                next: None,
            })),
        })),
    };
    capture(&node);

    let node = RCCNode {
        i: 11,
        next: Some(Arc::new(RCCNode {
            i: 22,
            next: Some(Arc::new(RCCNode {
                i: 33,
                next: None,
            })),
        })),
    };
    capture(&node);
}
