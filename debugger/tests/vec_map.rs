mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream, PValue, RValue};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "vec_map";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..22 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match event {
            Event::Breakpoint { .. } => unreachable!(),
            Event::FunctionCall { arguments, .. } => {
                if i == 0 {
                    assert_eq!(arguments.len(), 0);
                } else {
                    assert_eq!(arguments.len(), 1);
                    assert_eq!(arguments[0].1, RValue::Prim(PValue::i32(i / 2)));
                }
            }
            Event::FunctionReturn { return_value, .. } => {
                if i == 21 {
                    assert_eq!(return_value, RValue::Unit);
                } else {
                    assert_eq!(return_value, RValue::Prim(PValue::i32(square(i / 2 - 1))));
                }
            }
        }
    }

    Ok(())
}

fn square(v: i32) -> i32 {
    v * v
}
