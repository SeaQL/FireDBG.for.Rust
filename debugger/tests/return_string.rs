mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_string";
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
            name: "hello".into(),
            args: vec!["22i32".into()],
        },
        Expected::FnRet {
            name: "hello".into(),
            value: "String::from(\"hello 22\")".into(),
        },
        Expected::FnCall {
            name: "world".into(),
            args: vec![],
        },
        Expected::FnCall {
            name: "hello".into(),
            args: vec!["11i32".into()],
        },
        Expected::FnRet {
            name: "hello".into(),
            value: "String::from(\"hello 11\")".into(),
        },
        Expected::FnRet {
            name: "world".into(),
            value: "String::from(\"hello 11\")".into(),
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
