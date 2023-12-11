mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "gen_assoc_type";
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
                        0 => "gen_assoc_type::main",
                        1 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::contains",
                        3 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::first",
                        5 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::last",
                        7 => "gen_assoc_type::difference",
                        8 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::last",
                        10 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::first",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::contains",
                        4 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::first",
                        6 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::last",
                        9 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::last",
                        11 => "<gen_assoc_type::Container as gen_assoc_type::Contains>::first",
                        12 => "gen_assoc_type::difference",
                        13 => "gen_assoc_type::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
