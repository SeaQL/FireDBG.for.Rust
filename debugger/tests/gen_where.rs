mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "gen_where";
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
                        0 => "gen_where::main",
                        1 => "<T as gen_where::PrintInOption>::print_in_option",
                        3 => "<T as gen_where::PrintInOption>::print_in_option",
                        5 => "<T as gen_where::PrintInOption>::print_in_option",
                        7 => "<T as gen_where::PrintInOption>::print_in_option",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "<T as gen_where::PrintInOption>::print_in_option",
                        4 => "<T as gen_where::PrintInOption>::print_in_option",
                        6 => "<T as gen_where::PrintInOption>::print_in_option",
                        8 => "<T as gen_where::PrintInOption>::print_in_option",
                        9 => "gen_where::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
