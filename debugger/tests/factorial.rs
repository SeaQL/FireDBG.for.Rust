mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream, PValue, RValue};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "factorial";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let mut cc = 1;
    let mut j = 0;
    for i in 0..14 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match event {
            Event::Breakpoint { .. } => unreachable!(),
            Event::FunctionCall {
                function_name,
                arguments,
                ..
            } => {
                assert_eq!(
                    &function_name,
                    match i {
                        0 => "factorial::main",
                        _ => "factorial::factorial",
                    }
                );
                if i == 0 {
                    assert_eq!(arguments.len(), 0);
                } else {
                    assert_eq!(arguments.len(), 1);
                }
            }
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => {
                assert_eq!(
                    &function_name,
                    match i {
                        13 => "factorial::main",
                        _ => "factorial::factorial",
                    }
                );
                if i == 13 {
                    assert_eq!(return_value, RValue::Unit);
                } else {
                    assert_eq!(return_value, RValue::Prim(PValue::i32(cc)));
                    j += 1;
                    cc *= j;
                }
            }
        }
    }

    Ok(())
}
