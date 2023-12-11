mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "thread_2";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let mut thread_0 = None;
    let mut thread_1 = None;
    let mut thread_1_count = 0;
    let mut thread_2 = None;
    let mut thread_2_count = 0;

    for i in 0..40 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall {
                thread_id,
                function_name,
                ..
            } => {
                if function_name == "thread_2::main" {
                    thread_0 = Some(*thread_id);
                } else if function_name == "thread_2::run" {
                    assert_eq!(thread_0, Some(*thread_id));
                } else if function_name == "thread_2::runner" {
                    if thread_1.is_none() {
                        thread_1 = Some(*thread_id);
                    } else if thread_2.is_none() {
                        thread_2 = Some(*thread_id);
                    } else {
                        panic!("unexpected thread id {thread_id}");
                    }
                } else {
                    assert_ne!(thread_0, Some(*thread_id));
                    assert_eq!(function_name, "thread_2::tick");

                    if thread_1 == Some(*thread_id) {
                        thread_1_count += 1;
                    } else if thread_2 == Some(*thread_id) {
                        thread_2_count += 1;
                    } else {
                        panic!("unexpected thread id {thread_id}");
                    }
                }
            }
            Event::FunctionReturn {
                thread_id,
                function_name,
                ..
            } => {
                if function_name == "thread_2::main" {
                    assert_eq!(thread_0, Some(*thread_id));
                } else if function_name == "thread_2::run" {
                    assert_eq!(thread_0, Some(*thread_id));
                } else {
                    assert_ne!(thread_0, Some(*thread_id));

                    if function_name == "thread_2::runner" {
                        // good
                    } else {
                        assert_eq!(function_name, "thread_2::tick");
                    }

                    if thread_1 == Some(*thread_id) || thread_2 == Some(*thread_id) {
                        // good
                    } else {
                        panic!("unexpected thread id {thread_id}");
                    }
                }
            }
        }
    }

    if thread_1_count == 5 {
        assert_eq!(thread_2_count, 10);
    } else if thread_2_count == 5 {
        assert_eq!(thread_1_count, 10);
    } else {
        panic!("thread_1_count = {thread_1_count}, thread_2_count = {thread_2_count}");
    }

    Ok(())
}
