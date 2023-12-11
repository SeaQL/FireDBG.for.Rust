mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "thread_1";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let mut thread_0 = None;
    let mut thread_1 = None;
    let mut tick = 0;

    for i in 0..26 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall {
                thread_id,
                function_name,
                arguments,
                ..
            } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => {
                            thread_0 = Some(*thread_id);
                            "thread_1::main"
                        }
                        1 => {
                            assert_eq!(thread_0, Some(*thread_id));
                            "thread_1::run"
                        }
                        2 => {
                            thread_1 = Some(*thread_id);
                            "thread_1::thread_1"
                        }
                        _ => {
                            assert_eq!(arguments[0].1.to_string(), format!("{tick}usize"));
                            assert_eq!(thread_1, Some(*thread_id));
                            tick += 1;
                            "thread_1::tick"
                        }
                    }
                );
            }
            Event::FunctionReturn {
                thread_id,
                function_name,
                return_value,
                ..
            } => {
                assert_eq!(return_value.to_string().as_str(), "()");
                assert_eq!(
                    function_name,
                    match i {
                        23 => {
                            assert_eq!(thread_1, Some(*thread_id));
                            "thread_1::thread_1"
                        }
                        24 => {
                            assert_eq!(thread_0, Some(*thread_id));
                            "thread_1::run"
                        }
                        25 => {
                            assert_eq!(thread_0, Some(*thread_id));
                            "thread_1::main"
                        }
                        _ => {
                            assert_eq!(thread_1, Some(*thread_id));
                            "thread_1::tick"
                        }
                    }
                );
            }
        }
    }

    Ok(())
}
