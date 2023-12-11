mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, Event, EventStream, LineColumn, RValue, Reason,
    VariableCapture, RUST_PANIC_BP_ID,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "fn_call_panic";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    assert_eq!(
        debugger_params.breakpoints[1],
        Breakpoint {
            id: 1,
            file_id: 1,
            loc: LineColumn {
                line: 8,
                column: Some(27),
            },
            loc_end: Some(LineColumn {
                line: 9,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "hello_1".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );
    assert_eq!(
        debugger_params.breakpoints[2],
        Breakpoint {
            id: 2,
            file_id: 1,
            loc: LineColumn {
                line: 12,
                column: Some(28),
            },
            loc_end: Some(LineColumn {
                line: 13,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "hello_2".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );
    assert_eq!(
        debugger_params.breakpoints[3],
        Breakpoint {
            id: 3,
            file_id: 1,
            loc: LineColumn {
                line: 16,
                column: Some(28),
            },
            loc_end: Some(LineColumn {
                line: 17,
                column: Some(5),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "hello_3".into()
            },
            capture: VariableCapture::Arguments,
        },
    );
    assert_eq!(
        debugger_params.breakpoints[4],
        Breakpoint {
            id: 4,
            file_id: 1,
            loc: LineColumn {
                line: 22,
                column: Some(63),
            },
            loc_end: Some(LineColumn {
                line: 23,
                column: Some(9)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "fmt".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );
    assert_eq!(
        debugger_params.breakpoints[5],
        Breakpoint {
            id: 5,
            file_id: 1,
            loc: LineColumn {
                line: 27,
                column: Some(12),
            },
            loc_end: Some(LineColumn {
                line: 28,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "main".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let the_world = r#"{"type":"Struct","typename":"fn_call_panic::World","fields":{"nth":{"type":"Prim","typename":"i32","value":99}}}"#;

    for i in 0..11 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {event:?}");
        match event {
            Event::FunctionCall {
                function_name,
                arguments: values,
                ..
            } => match i {
                0 => {
                    assert_eq!(values.len(), 0);
                    assert_eq!(function_name, "fn_call_panic::main");
                }
                1 => {
                    assert_eq!(values.len(), 1);
                    assert_eq!(function_name, "fn_call_panic::hello_1");
                    let (name, value) = &values[0];
                    let json = serde_json::to_string(value).unwrap();
                    println!("{name} = {json}");
                    assert_eq!(json, the_world);
                }
                2 | 6 => {
                    assert_eq!(values.len(), 2);
                    assert_eq!(
                        function_name,
                        "<fn_call_panic::World as core::fmt::Display>::fmt"
                    );
                    let (name, value) = &values[0];
                    if let RValue::Ref { value, .. } = value {
                        let json = serde_json::to_string(value).unwrap();
                        println!("{name}  = {json}");
                        assert_eq!(json, the_world);
                    } else {
                        panic!("{value:?}");
                    }
                }
                5 => {
                    assert_eq!(values.len(), 1);
                    assert_eq!(function_name, "fn_call_panic::hello_2");
                    let (name, value) = &values[0];
                    if let RValue::Ref { value, .. } = value {
                        let json = serde_json::to_string(value).unwrap();
                        println!("{name} = {json}");
                        assert_eq!(json, the_world);
                    } else {
                        panic!("{value:?}");
                    }
                }
                9 => {
                    assert_eq!(values.len(), 1);
                    assert_eq!(function_name, "fn_call_panic::hello_3");
                    let (name, value) = &values[0];
                    if let RValue::Ref { value, .. } = value {
                        let json = serde_json::to_string(value).unwrap();
                        println!("{name} = {json}");
                        assert_eq!(json, the_world);
                    } else {
                        panic!("{value:?}");
                    }
                }
                other => panic!("Unexpected {other:?}"),
            },
            Event::FunctionReturn { function_name, .. } => match i {
                3 | 7 => {
                    assert_eq!(
                        function_name,
                        "<fn_call_panic::World as core::fmt::Display>::fmt"
                    )
                }
                4 => assert_eq!(function_name, "fn_call_panic::hello_1"),
                8 => assert_eq!(function_name, "fn_call_panic::hello_2"),
                9 => assert_eq!(function_name, "fn_call_panic::main"),
                other => panic!("Unexpected {other:?}"),
            },
            Event::Breakpoint {
                breakpoint_id,
                reason,
                locals: values,
                ..
            } => {
                assert_eq!(i, 10);
                assert_eq!(breakpoint_id, RUST_PANIC_BP_ID.0);
                assert_eq!(reason, Reason::Panic);
                assert_eq!(values.len(), 1);
                let (name, value) = &values[0];
                if let RValue::Ref { value, .. } = value {
                    let json = serde_json::to_string(value).unwrap();
                    println!("{name} = {json}");
                    assert_eq!(json, the_world);
                } else {
                    panic!("{value:?}");
                }
            }
        }
    }

    Ok(())
}
