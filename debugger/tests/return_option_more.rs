mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_option_more";
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
            name: "usize_pair".into(),
            args: vec!["0i32".to_owned()],
        },
        Expected::FnRet {
            name: "usize_pair".into(),
            value: expand("Option::<(usize, usize)>::None"),
        },
        Expected::FnCall {
            name: "usize_pair".into(),
            args: vec!["1i32".to_owned()],
        },
        Expected::FnRet {
            name: "usize_pair".into(),
            value: expand("Option::<(usize, usize)>::Some((1234usize, 5678usize))"),
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
    string.replace("Option", "core::option::Option")
}
