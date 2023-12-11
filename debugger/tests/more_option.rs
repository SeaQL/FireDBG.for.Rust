mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "more_option";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let expected = vec![
        Expected::FnCall {
            name: "main".into(),
            args: vec![],
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&BNode { i: 1i32, next: Option<Box<BNode>>::Some(Box::new(BNode { i: 2i32, next: Option<Box<BNode>>::None })) }")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&BNode { i: 1i32, next: Option<Box<BNode>>::Some(Box::new(BNode { i: 2i32, next: Option<Box<BNode>>::Some(Box::new(BNode { i: 3i32, next: Option<Box<BNode>>::None })) })) }")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "&alloc::boxed::Box::<dyn core::fmt::Debug>::new(\"hello\")".to_owned()
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&Option<Box<dyn core::fmt::Debug>>::Some(Box::<dyn core::fmt::Debug>::new(\"hello\"))")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&RCNode { i: 11i32, next: Option<Rc<RCNode>>::None }")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&RCNode { i: 11i32, next: Option<Rc<RCNode>>::Some(Rc::new(RCNode { i: 22i32, next: Option<Rc<RCNode>>::None })) }")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&RCNode { i: 11i32, next: Option<Rc<RCNode>>::Some(Rc::new(RCNode { i: 22i32, next: Option<Rc<RCNode>>::Some(Rc::new(RCNode { i: 33i32, next: Option<Rc<RCNode>>::None })) })) }")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                expand("&RCCNode { i: 11i32, next: Option<Arc<RCCNode>>::Some(Arc::new(RCCNode { i: 22i32, next: Option<Arc<RCCNode>>::Some(Arc::new(RCCNode { i: 33i32, next: Option<Arc<RCCNode>>::None })) })) }")
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "()".into(),
        },
        Expected::FnRet {
            name: "main".into(),
            value: "()".into(),
        },
    ];

    let mut events = Vec::new();
    for i in 0..expected.len() {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);
        events.push(event);
    }

    verify(testcase, events, expected);

    Ok(())
}

fn expand(string: &str) -> String {
    string
        .replace("BNode", "more_option::BNode")
        .replace("RCNode", "more_option::RCNode")
        .replace("RCCNode", "more_option::RCCNode")
        .replace("Option<", "core::option::Option::<")
        .replace("Box", "alloc::boxed::Box")
        .replace("Rc", "alloc::rc::Rc")
        .replace("Arc", "alloc::sync::Arc")
}
