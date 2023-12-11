mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, Event, EventStream, LineColumn, VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "result";
    let (producer, consumer) = setup(testcase).await?;

    let mut debugger_params = debugger_params_from_file(testcase);
    debugger_params.breakpoints.push(Breakpoint {
        id: debugger_params.breakpoints.len() as u32,
        file_id: 1,
        loc: LineColumn {
            line: 21,
            column: None,
        },
        loc_end: None,
        breakpoint_type: BreakpointType::Breakpoint,
        capture: VariableCapture::Only(vec!["ok".to_owned(), "err".to_owned()]),
    });

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;
    for i in 0..7 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        match event {
            Event::FunctionCall {
                function_name,
                arguments,
                ..
            } => {
                if i == 0 {
                    assert_eq!(arguments.len(), 0);
                    continue;
                }
                assert_eq!(arguments.len(), 1);
                let value = arguments.into_iter().next().unwrap().1;
                let value = value.to_string();
                match i {
                    1 => assert_eq!(
                        value,
                        r#"core::result::Result::<result::Good, result::Bad>::Err(result::Bad { i: 1234u32 })"#
                    ),
                    3 => assert_eq!(
                        value,
                        r#"core::result::Result::<result::Good, result::Bad>::Ok(result::Good(5678i32))"#
                    ),
                    _ => panic!("Unexpected i {i}"),
                }
                println!("[{i}] {function_name}({value})");
            }
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => {
                let value = return_value.to_string();
                match i {
                    2 => assert_eq!(
                        value,
                        r#"core::result::Result::<result::Good, result::Bad>::Ok(1235i32)"#
                    ),
                    4 => assert_eq!(
                        value,
                        r#"core::result::Result::<result::Good, result::Bad>::Err(5679u32)"#
                    ),
                    6 => assert_eq!(value, r#"()"#),
                    _ => panic!("Unexpected i {i}"),
                }
                println!("[{i}] {function_name}() -> {value}");
            }
            Event::Breakpoint { locals, .. } => {
                assert_eq!(i, 5);
                assert_eq!(locals.len(), 2);
                for (i, (name, value)) in locals.iter().enumerate() {
                    match i {
                        0 => {
                            assert_eq!(name.as_str(), "ok");
                            let json = serde_json::to_string(value).unwrap();
                            println!("{name} = {json}");
                            assert_eq!(
                                json,
                                r#"{"type":"Result","typename":"core::result::Result<result::Good, result::Bad>","variant":"Ok","value":{"type":"Struct","typename":"result::Good","fields":{"0":{"type":"Prim","typename":"i32","value":1235}}}}"#
                            );
                        }
                        1 => {
                            assert_eq!(name.as_str(), "err");
                            let json = serde_json::to_string(value).unwrap();
                            println!("{name} = {json}");
                            assert_eq!(
                                json,
                                r#"{"type":"Result","typename":"core::result::Result<result::Good, result::Bad>","variant":"Err","value":{"type":"Struct","typename":"result::Bad","fields":{"i":{"type":"Prim","typename":"u32","value":5679}}}}"#
                            );
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    Ok(())
}
