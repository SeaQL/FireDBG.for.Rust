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
    let testcase = "fn_name";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    assert_eq!(
        debugger_params.breakpoints[0],
        Breakpoint {
            id: 0,
            file_id: 0,
            loc: LineColumn {
                line: 0,
                column: None,
            },
            loc_end: None,
            breakpoint_type: BreakpointType::Breakpoint,
            capture: VariableCapture::None,
        }
    );
    assert_eq!(
        debugger_params.breakpoints[1],
        Breakpoint {
            id: 1,
            file_id: 1,
            loc: LineColumn {
                line: 12,
                column: Some(71),
            },
            loc_end: Some(LineColumn {
                line: 13,
                column: Some(13)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "display".into(),
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
                line: 18,
                column: Some(67),
            },
            loc_end: Some(LineColumn {
                line: 19,
                column: Some(13)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "fmt".into(),
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
                line: 29,
                column: Some(71),
            },
            loc_end: Some(LineColumn {
                line: 30,
                column: Some(13)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "display".into(),
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
                line: 35,
                column: Some(67),
            },
            loc_end: Some(LineColumn {
                line: 36,
                column: Some(13)
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
                line: 43,
                column: Some(45),
            },
            loc_end: Some(LineColumn {
                line: 44,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "inlined_display".into(),
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
                line: 47,
                column: Some(12),
            },
            loc_end: Some(LineColumn {
                line: 48,
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

    for i in 0..22 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {
                println!("FunctionCall {function_name}");
                assert_eq!(
                    function_name,
                    match i {
                        0 => "fn_name::main",
                        1 => "<fn_name::pet::Cat as core::fmt::Display>::fmt",
                        2 => "fn_name::pet::Cat::display",
                        5 => "fn_name::inlined_display",
                        6 => "<fn_name::pet::Cat as core::fmt::Display>::fmt",
                        7 => "fn_name::pet::Cat::display",
                        11 => "<fn_name::pet::Dog as core::fmt::Display>::fmt",
                        12 => "fn_name::pet::Dog::display",
                        15 => "fn_name::inlined_display",
                        16 => "<fn_name::pet::Dog as core::fmt::Display>::fmt",
                        17 => "fn_name::pet::Dog::display",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn { function_name, .. } => {
                println!("FunctionReturn {function_name}");
                assert_eq!(
                    function_name,
                    match i {
                        3 => "fn_name::pet::Cat::display",
                        4 => "<fn_name::pet::Cat as core::fmt::Display>::fmt",
                        8 => "fn_name::pet::Cat::display",
                        9 => "<fn_name::pet::Cat as core::fmt::Display>::fmt",
                        10 => "fn_name::inlined_display",
                        13 => "fn_name::pet::Dog::display",
                        14 => "<fn_name::pet::Dog as core::fmt::Display>::fmt",
                        18 => "fn_name::pet::Dog::display",
                        19 => "<fn_name::pet::Dog as core::fmt::Display>::fmt",
                        20 => "fn_name::inlined_display",
                        21 => "fn_name::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
