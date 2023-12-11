mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, Event, EventStream, LineColumn, RValue,
    VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "fn_call";
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
                line: 17,
                column: Some(63),
            },
            loc_end: Some(LineColumn {
                line: 18,
                column: Some(9)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "fmt".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );
    assert_eq!(
        debugger_params.breakpoints[4],
        Breakpoint {
            id: 4,
            file_id: 1,
            loc: LineColumn {
                line: 22,
                column: Some(12),
            },
            loc_end: Some(LineColumn {
                line: 23,
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

    let the_world = r#"{"type":"Struct","typename":"fn_call::World","fields":{"nth":{"type":"Prim","typename":"i32","value":99}}}"#;

    for i in 0..10 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        match event {
            Event::FunctionCall {
                function_name,
                arguments: values,
                ..
            } => {
                println!("#{i} FunctionCall `{function_name}`");
                match i {
                    0 => {
                        assert_eq!(values.len(), 0);
                        assert_eq!(function_name, "fn_call::main");
                    }
                    1 => {
                        assert_eq!(values.len(), 1);
                        assert_eq!(function_name, "fn_call::hello_1");
                        let (name, value) = &values[0];
                        let json = serde_json::to_string(value).unwrap();
                        println!("{name} = {json}");
                        assert_eq!(json, the_world);
                    }
                    2 | 6 => {
                        assert_eq!(values.len(), 2);
                        assert_eq!(function_name, "<fn_call::World as core::fmt::Display>::fmt");
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
                        assert_eq!(function_name, "fn_call::hello_2");
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
                }
            }
            Event::FunctionReturn { function_name, .. } => {
                println!("#{i} FunctionReturn `{function_name}`");
                match i {
                    3 | 7 => {
                        assert_eq!(function_name, "<fn_call::World as core::fmt::Display>::fmt")
                    }
                    4 => assert_eq!(function_name, "fn_call::hello_1"),
                    8 => assert_eq!(function_name, "fn_call::hello_2"),
                    9 => assert_eq!(function_name, "fn_call::main"),
                    other => panic!("Unexpected {other:?}"),
                }
            }
            other => panic!("Unexpected {other:?}"),
        }
    }

    Ok(())
}
