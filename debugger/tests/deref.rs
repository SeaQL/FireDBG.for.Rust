mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "deref";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..20 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        // match &event {
        //     Event::Breakpoint { .. } => (),
        //     Event::FunctionCall { function_name, .. } => {
        //         assert_eq!(
        //             function_name,
        //             match i {
        //                 0 => "gen_fn::main",
        //                 1 => "gen_fn::reg_fn",
        //                 3 => "gen_fn::gen_spec_t",
        //                 5 => "gen_fn::gen_spec_i32",
        //                 7 => "gen_fn::generic",
        //                 9 => "gen_fn::generic",
        //                 _ => panic!("Unexpected i {i}"),
        //             }
        //         );
        //     }
        //     Event::FunctionReturn { function_name, .. } => {
        //         assert_eq!(
        //             function_name,
        //             match i {
        //                 2 => "gen_fn::reg_fn",
        //                 4 => "gen_fn::gen_spec_t",
        //                 6 => "gen_fn::gen_spec_i32",
        //                 8 => "gen_fn::generic",
        //                 10 => "gen_fn::generic",
        //                 11 => "gen_fn::main",
        //                 _ => panic!("Unexpected i {i}"),
        //             }
        //         );
        //     }
        // }
    }

    Ok(())
}
