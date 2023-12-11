mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "dyn_box";
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
            name: expand("<MyStruct as MyTrait>::i"),
            args: vec![expand("&MyStruct { i: 1234i32 }")],
        },
        Expected::FnRet {
            name: expand("<MyStruct as MyTrait>::i"),
            value: "1234i32".into(),
        },
        Expected::FnCall {
            name: "open".into(),
            args: vec![expand("&Box::<dyn MyTrait>::new(MyStruct { i: 1234i32 })")],
        },
        Expected::FnRet {
            name: "open".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: expand("<MyStruct as MyTrait>::i"),
            args: vec![expand("&MyStruct { i: 1234i32 }")],
        },
        Expected::FnRet {
            name: expand("<MyStruct as MyTrait>::i"),
            value: "1234i32".into(),
        },
        Expected::FnCall {
            name: "open".into(),
            args: vec![expand("&Rc::<dyn MyTrait>::new(MyStruct { i: 1234i32 })")],
        },
        Expected::FnRet {
            name: "open".into(),
            value: "()".into(),
        },
        Expected::FnCall {
            name: expand("<MyOther as MyTrait>::i"),
            args: vec![expand("&MyOther { not_i: 5678i64 }")],
        },
        Expected::FnRet {
            name: expand("<MyOther as MyTrait>::i"),
            value: "5678i32".into(),
        },
        Expected::FnCall {
            name: "open".into(),
            args: vec![expand(
                "&Arc::<dyn MyTrait>::new(MyOther { not_i: 5678i64 })",
            )],
        },
        Expected::FnRet {
            name: "open".into(),
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
        .replace("MyTrait", "dyn_box::MyTrait")
        .replace("MyStruct", "dyn_box::MyStruct")
        .replace("MyOther", "dyn_box::MyOther")
        .replace("Box", "alloc::boxed::Box")
        .replace("Rc", "alloc::rc::Rc")
        .replace("Arc", "alloc::sync::Arc")
}
