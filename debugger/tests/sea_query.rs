mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "sea_query";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..46 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);
        match event {
            Event::FunctionCall {
                function_name,
                mut arguments,
                ..
            } => {
                arguments.iter_mut().for_each(|(_, val)| val.redact_addr());
                let json = serde_json::to_string(&arguments).unwrap();
                match i {
                    0 => assert_eq!(function_name, "sea_query::main"),
                    1 => assert_eq!(function_name, "sea_query::Query::select"),
                    2 => assert_eq!(function_name, "sea_query::SelectStatement::new"),
                    _ => (),
                }
                match i {
                    0..=2 => assert_eq!(json, r#"[]"#),
                    _ => (),
                }
                println!("[{i}] {function_name}() -> {arguments:?}");
            }
            Event::FunctionReturn {
                function_name,
                mut return_value,
                ..
            } => {
                return_value.redact_addr();
                let json = serde_json::to_string(&return_value).unwrap();
                match i {
                    3 => assert_eq!(function_name, "sea_query::SelectStatement::new"),
                    4 => assert_eq!(function_name, "sea_query::Query::select"),
                    45 => assert_eq!(function_name, "sea_query::main"),
                    _ => (),
                }
                match i {
                    3 => assert_eq!(
                        json,
                        r#"{"type":"Struct","typename":"sea_query::SelectStatement","fields":{"selects":{"type":"Array","typename":"vec","data":[]},"from":{"type":"Array","typename":"vec","data":[]}}}"#
                    ),
                    4 => assert_eq!(
                        json,
                        r#"{"type":"Struct","typename":"sea_query::SelectStatement","fields":{"selects":{"type":"Array","typename":"vec","data":[]},"from":{"type":"Array","typename":"vec","data":[]}}}"#
                    ),
                    45 => assert_eq!(json, r#"{"type":"Unit"}"#),
                    _ => (),
                }
                println!("[{i}] {function_name}() -> {return_value}");
            }
            e => panic!("Unexpected {e:?}"),
        }
    }

    Ok(())
}
