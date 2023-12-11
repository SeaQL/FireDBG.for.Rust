mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream, PValue, RValue};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "call_chain";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..10 {
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
                        0 => "call_chain::main",
                        1 => "call_chain::head",
                        2 => "call_chain::inter",
                        3 => "call_chain::tail",
                        4 => "call_chain::end",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                if i == 0 {
                    assert_eq!(arguments.len(), 0);
                } else {
                    assert_eq!(arguments.len(), 1);
                    assert_eq!(
                        arguments[0].1,
                        RValue::Prim(PValue::i32(match i {
                            1 => 1,
                            2 => 2,
                            3 => 2,
                            4 => 2,
                            _ => panic!("Unexpected i {i}"),
                        }))
                    );
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
                        5 => "call_chain::end",
                        6 => "call_chain::tail",
                        7 => "call_chain::inter",
                        8 => "call_chain::head",
                        9 => "call_chain::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                if i == 9 {
                    assert_eq!(return_value, RValue::Unit);
                } else {
                    assert_eq!(
                        return_value,
                        RValue::Prim(PValue::i32(match i {
                            5 => 2,
                            6 => 3,
                            7 => 3,
                            8 => 3,
                            _ => panic!("Unexpected i {i}"),
                        }))
                    );
                }
            }
        }
    }

    Ok(())
}
