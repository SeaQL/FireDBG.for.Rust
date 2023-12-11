mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "gen_new_type";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..10 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "gen_new_type::main",
                        1 => "gen_new_type::Years::to_days",
                        3 => "gen_new_type::old_enough",
                        5 => "gen_new_type::Days::to_years",
                        7 => "gen_new_type::old_enough",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "gen_new_type::Years::to_days",
                        4 => "gen_new_type::old_enough",
                        6 => "gen_new_type::Days::to_years",
                        8 => "gen_new_type::old_enough",
                        9 => "gen_new_type::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
