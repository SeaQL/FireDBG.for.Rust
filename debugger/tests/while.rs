mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "while";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..8 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "while::main",
                        1 => "while::run",
                        2 => "while::iter",
                        4 => "while::iter",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        3 => "while::iter",
                        5 => "while::iter",
                        6 => "while::run",
                        7 => "while::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
