mod util;
use util::*;

use std::collections::HashMap;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, Event, EventStream, LineColumn, PValue, RValue,
    StringType, VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_value";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    assert_eq!(
        debugger_params.breakpoints[1],
        Breakpoint {
            id: 1,
            file_id: 1,
            loc: LineColumn {
                line: 1,
                column: Some(21),
            },
            loc_end: Some(LineColumn {
                line: 1,
                column: Some(22),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "u8".into(),
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
                line: 2,
                column: Some(21),
            },
            loc_end: Some(LineColumn {
                line: 2,
                column: Some(22),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "i8".into(),
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
                line: 3,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 3,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "u16".into(),
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
                line: 4,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 4,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "i16".into(),
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
                line: 5,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 5,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "u32".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[6],
        Breakpoint {
            id: 6,
            file_id: 1,
            loc: LineColumn {
                line: 6,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 6,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "i32".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[7],
        Breakpoint {
            id: 7,
            file_id: 1,
            loc: LineColumn {
                line: 7,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 7,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "u64".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[8],
        Breakpoint {
            id: 8,
            file_id: 1,
            loc: LineColumn {
                line: 8,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 8,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "i64".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[9],
        Breakpoint {
            id: 9,
            file_id: 1,
            loc: LineColumn {
                line: 9,
                column: Some(27),
            },
            loc_end: Some(LineColumn {
                line: 9,
                column: Some(28),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "u128".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[10],
        Breakpoint {
            id: 10,
            file_id: 1,
            loc: LineColumn {
                line: 10,
                column: Some(27),
            },
            loc_end: Some(LineColumn {
                line: 10,
                column: Some(28),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "i128".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[11],
        Breakpoint {
            id: 11,
            file_id: 1,
            loc: LineColumn {
                line: 11,
                column: Some(30),
            },
            loc_end: Some(LineColumn {
                line: 11,
                column: Some(31),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "usize".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[12],
        Breakpoint {
            id: 12,
            file_id: 1,
            loc: LineColumn {
                line: 12,
                column: Some(30),
            },
            loc_end: Some(LineColumn {
                line: 12,
                column: Some(31),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "isize".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[13],
        Breakpoint {
            id: 13,
            file_id: 1,
            loc: LineColumn {
                line: 13,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 13,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "f32".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[14],
        Breakpoint {
            id: 14,
            file_id: 1,
            loc: LineColumn {
                line: 14,
                column: Some(24),
            },
            loc_end: Some(LineColumn {
                line: 14,
                column: Some(25),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "f64".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[15],
        Breakpoint {
            id: 15,
            file_id: 1,
            loc: LineColumn {
                line: 15,
                column: Some(27),
            },
            loc_end: Some(LineColumn {
                line: 15,
                column: Some(28),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "bool".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[16],
        Breakpoint {
            id: 16,
            file_id: 1,
            loc: LineColumn {
                line: 16,
                column: Some(49),
            },
            loc_end: Some(LineColumn {
                line: 16,
                column: Some(50),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "static_str".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[17],
        Breakpoint {
            id: 17,
            file_id: 1,
            loc: LineColumn {
                line: 17,
                column: Some(43),
            },
            loc_end: Some(LineColumn {
                line: 17,
                column: Some(44),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "result_ok".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[18],
        Breakpoint {
            id: 18,
            file_id: 1,
            loc: LineColumn {
                line: 18,
                column: Some(44),
            },
            loc_end: Some(LineColumn {
                line: 18,
                column: Some(45),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "result_err".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[19],
        Breakpoint {
            id: 19,
            file_id: 1,
            loc: LineColumn {
                line: 19,
                column: Some(46),
            },
            loc_end: Some(LineColumn {
                line: 19,
                column: Some(47),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "result_ok_u32".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[20],
        Breakpoint {
            id: 20,
            file_id: 1,
            loc: LineColumn {
                line: 20,
                column: Some(47),
            },
            loc_end: Some(LineColumn {
                line: 20,
                column: Some(48),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "result_err_u64".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    assert_eq!(
        debugger_params.breakpoints[21],
        Breakpoint {
            id: 21,
            file_id: 1,
            loc: LineColumn {
                line: 22,
                column: Some(12),
            },
            loc_end: Some(LineColumn {
                line: 23,
                column: Some(5),
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "main".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;
    let mut hashmap = HashMap::new();

    for i in 0..108 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match event {
            Event::Breakpoint { .. } => unreachable!(),
            Event::FunctionCall {
                thread_id,
                frame_id,
                function_name,
                arguments,
                ..
            } => {
                let arg = if function_name == "return_value::main" {
                    assert_eq!(arguments.len(), 0);
                    RValue::Unit
                } else {
                    assert_eq!(arguments.len(), 1);
                    let (_, v) = &arguments[0];
                    v.clone()
                };
                hashmap
                    .entry((thread_id, frame_id))
                    .or_insert((function_name, arg));
            }
            Event::FunctionReturn {
                breakpoint_id: _,
                thread_id,
                frame_id,
                function_name,
                return_value,
            } => {
                let (fn_call_function_name, fn_call_arg) = hashmap
                    .remove(&(thread_id, frame_id))
                    .expect("FunctionCall not found");
                assert_eq!(fn_call_function_name, function_name);
                match i {
                    98 => {
                        assert_eq!(
                            fn_call_arg,
                            RValue::String {
                                typename: StringType::StrLit,
                                value: "hi".to_owned()
                            }
                        );
                        assert_eq!(
                            return_value,
                            RValue::String {
                                typename: StringType::StrLit,
                                value: "hello".to_owned()
                            }
                        );
                    }
                    100 => {
                        assert_eq!(
                            return_value,
                            RValue::Result {
                                typename: "core::result::Result<i64, i64>".to_owned(),
                                variant: "Ok".to_owned(),
                                value: Box::new(RValue::Prim(PValue::i64(0x1234))),
                            }
                        );
                    }
                    102 => {
                        assert_eq!(
                            return_value,
                            RValue::Result {
                                typename: "core::result::Result<i32, i32>".to_owned(),
                                variant: "Err".to_owned(),
                                value: Box::new(RValue::Prim(PValue::i32(0x5678))),
                            }
                        );
                    }
                    104 => {
                        assert_eq!(
                            return_value,
                            RValue::Result {
                                typename: "core::result::Result<u32, ()>".to_owned(),
                                variant: "Ok".to_owned(),
                                value: Box::new(RValue::Prim(PValue::u32(1234))),
                            }
                        );
                    }
                    106 => {
                        assert_eq!(
                            return_value,
                            RValue::Result {
                                typename: "core::result::Result<(), u64>".to_owned(),
                                variant: "Err".to_owned(),
                                value: Box::new(RValue::Prim(PValue::u64(12345678))),
                            }
                        );
                    }
                    _ => {
                        assert_eq!(fn_call_arg, return_value);
                    }
                }
            }
        }
    }

    assert_eq!(hashmap.into_iter().collect::<Vec<_>>(), []);

    Ok(())
}
