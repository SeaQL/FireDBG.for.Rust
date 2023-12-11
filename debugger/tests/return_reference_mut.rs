mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_ref_mut";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..4 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);
        match event {
            Event::FunctionCall {
                function_name,
                arguments,
                ..
            } => {
                assert_eq!(arguments.len(), i);
                assert_eq!(
                    function_name,
                    match i {
                        0 => "return_ref_mut::main",
                        1 => "return_ref_mut::map_append",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                if i == 1 {
                    let mut value = arguments.into_iter().next().unwrap().1;
                    value.redact_addr();
                    let json = serde_json::to_string(&value).unwrap();
                    assert_eq!(
                        json,
                        r#"{"type":"Ref","typename":"ref","addr":"<redacted>","value":{"type":"Array","typename":"vec","data":[]}}"#
                    );
                }
            }
            Event::FunctionReturn {
                function_name,
                mut return_value,
                ..
            } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "return_ref_mut::map_append",
                        3 => "return_ref_mut::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                return_value.redact_addr();
                let json = serde_json::to_string(&return_value).unwrap();
                assert_eq!(
                    json,
                    match i {
                        2 =>
                            r#"{"type":"Ref","typename":"ref","addr":"<redacted>","value":{"type":"Array","typename":"vec","data":[{"type":"RefCounted","typename":"Rc","addr":"<redacted>","strong":1,"weak":1,"value":{"type":"Prim","typename":"i32","value":2}},{"type":"RefCounted","typename":"Rc","addr":"<redacted>","strong":1,"weak":1,"value":{"type":"Prim","typename":"i32","value":3}},{"type":"RefCounted","typename":"Rc","addr":"<redacted>","strong":1,"weak":1,"value":{"type":"Prim","typename":"i32","value":4}}]}}"#,
                        3 => r#"{"type":"Unit"}"#,
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
