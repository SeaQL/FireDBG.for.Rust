mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "gen_multi_bound";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..12 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "gen_multi_bound::main",
                        1 => "gen_multi_bound::compare_prints",
                        3 => "gen_multi_bound::compare_prints",
                        5 => "gen_multi_bound::compare_prints",
                        7 => "gen_multi_bound::compare_types",
                        9 => "gen_multi_bound::compare_types",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "gen_multi_bound::compare_prints",
                        4 => "gen_multi_bound::compare_prints",
                        6 => "gen_multi_bound::compare_prints",
                        8 => "gen_multi_bound::compare_types",
                        10 => "gen_multi_bound::compare_types",
                        11 => "gen_multi_bound::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
