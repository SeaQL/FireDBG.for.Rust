mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "gen_bound";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..14 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "gen_bound::main",
                        1 => "gen_bound::print_debug",
                        3 => "gen_bound::area",
                        4 => "<gen_bound::Rectangle as gen_bound::HasArea>::area",
                        7 => "gen_bound::print_debug",
                        9 => "gen_bound::area",
                        10 => "<gen_bound::Triangle as gen_bound::HasArea>::area",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "gen_bound::print_debug",
                        5 => "<gen_bound::Rectangle as gen_bound::HasArea>::area",
                        6 => "gen_bound::area",
                        8 => "gen_bound::print_debug",
                        11 => "<gen_bound::Triangle as gen_bound::HasArea>::area",
                        12 => "gen_bound::area",
                        13 => "gen_bound::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
