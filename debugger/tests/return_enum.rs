mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_enum";

    let debugger_params = debugger_params_from_file(testcase);

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..14 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);

        match (i, event) {
            (
                _,
                Event::FunctionCall {
                    function_name,
                    arguments,
                    ..
                },
            ) => {
                assert_eq!(
                    arguments.len(),
                    match i {
                        0 => 0,
                        _ => 1,
                    }
                );
                assert_eq!(
                    function_name,
                    match i {
                        0 => "return_enum::main",
                        1 => "return_enum::shrink",
                        3 => "return_enum::flip",
                        5 => "return_enum::flip",
                        7 => "return_enum::advance",
                        9 => "return_enum::advance",
                        11 => "return_enum::advance",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                if i == 0 {
                    println!("[{i}] {function_name}() ..");
                } else {
                    let (name, value) = &arguments[0];
                    println!("[{i}] {function_name}({name} = {value}) ..");
                }
            }
            (
                _,
                Event::FunctionReturn {
                    function_name,
                    return_value,
                    ..
                },
            ) => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "return_enum::shrink",
                        4 => "return_enum::flip",
                        6 => "return_enum::flip",
                        8 => "return_enum::advance",
                        10 => "return_enum::advance",
                        12 => "return_enum::advance",
                        13 => "return_enum::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                let json = serde_json::to_string(&return_value).unwrap();
                assert_eq!(
                    json,
                    match i {
                        2 => r#"{"type":"Enum","typename":"return_enum::Size","variant":"Medium"}"#,
                        4 =>
                            r#"{"type":"Enum","typename":"return_enum::Direction","variant":"South"}"#,
                        6 =>
                            r#"{"type":"Enum","typename":"return_enum::Direction","variant":"West"}"#,
                        8 => r#"{"type":"Enum","typename":"return_enum::Greek","variant":"Beta"}"#,
                        10 =>
                            r#"{"type":"Enum","typename":"return_enum::Greek","variant":"Gamma"}"#,
                        12 =>
                            r#"{"type":"Enum","typename":"return_enum::Greek","variant":"Delta"}"#,
                        13 => r#"{"type":"Unit"}"#,
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                println!("[{i}] {function_name}() -> {return_value}");
            }
            e => panic!("Unexpected {e:?}"),
        }
    }

    Ok(())
}
