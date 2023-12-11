mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "gen_phantom";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..6 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "gen_phantom::main",
                        1 => "<gen_phantom::Length<Unit> as core::ops::arith::Add>::add",
                        3 => "<gen_phantom::Length<Unit> as core::ops::arith::Add>::add",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "<gen_phantom::Length<Unit> as core::ops::arith::Add>::add",
                        4 => "<gen_phantom::Length<Unit> as core::ops::arith::Add>::add",
                        5 => "gen_phantom::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
