mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, DebuggerParams, Event, EventStream, LineColumn,
    SourceFile, VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};
use std::time::SystemTime;

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_object";

    rustc(&format!("testcases/{testcase}"));

    let debugger_params = DebuggerParams {
        binary: format!("testcases/{testcase}.o"),
        files: vec![
            Default::default(),
            SourceFile {
                id: 1,
                path: format!("testcases/{testcase}.rs"),
                crate_name: testcase.into(),
                modified: SystemTime::UNIX_EPOCH,
            },
        ],
        breakpoints: vec![
            Default::default(),
            Breakpoint {
                id: 1,
                file_id: 1,
                loc: LineColumn {
                    line: 60,
                    column: Some(12),
                },
                loc_end: Some(LineColumn {
                    line: 61,
                    column: Some(5),
                }),
                breakpoint_type: BreakpointType::FunctionCall {
                    fn_name: "main".into(),
                },
                capture: VariableCapture::Arguments,
            },
            Breakpoint {
                id: 2,
                file_id: 1,
                loc: LineColumn {
                    line: 72,
                    column: None,
                },
                loc_end: None,
                breakpoint_type: BreakpointType::Breakpoint,
                capture: VariableCapture::Only(vec!["car".to_owned()]),
            },
            Breakpoint {
                id: 3,
                file_id: 1,
                loc: LineColumn {
                    line: 29,
                    column: Some(32),
                },
                loc_end: Some(LineColumn {
                    line: 30,
                    column: Some(5),
                }),
                breakpoint_type: BreakpointType::FunctionCall {
                    fn_name: "create_manual_car".into(),
                },
                capture: VariableCapture::Arguments,
            },
            Breakpoint {
                id: 4,
                file_id: 1,
                loc: LineColumn {
                    line: 40,
                    column: Some(30),
                },
                loc_end: Some(LineColumn {
                    line: 41,
                    column: Some(5),
                }),
                breakpoint_type: BreakpointType::FunctionCall {
                    fn_name: "create_auto_car".into(),
                },
                capture: VariableCapture::Arguments,
            },
            Breakpoint {
                id: 5,
                file_id: 1,
                loc: LineColumn {
                    line: 52,
                    column: Some(64),
                },
                loc_end: Some(LineColumn {
                    line: 53,
                    column: Some(5),
                }),
                breakpoint_type: BreakpointType::FunctionCall {
                    fn_name: "choose_a_car_for_me".into(),
                },
                capture: VariableCapture::Arguments,
            },
        ],
        arguments: vec![],
    };

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..11 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);
        match (i, event) {
            (3, Event::Breakpoint { locals, .. }) => {
                let (name, value) = locals.iter().next().unwrap();
                assert_eq!(name.as_str(), "car");
                let json = serde_json::to_string(value).unwrap();
                assert_eq!(
                    json,
                    r#"{"type":"Struct","typename":"return_object::Car","fields":{"brand":{"type":"String","typename":"&str","value":"Nil"},"engine":{"type":"Struct","typename":"return_object::Engine","fields":{"config":{"type":"Union","typeinfo":{"name":"return_object::EngineConfig","variants":["Inline","Vshape"]},"variant":"Inline","fields":{"i":{"type":"Prim","typename":"i32","value":0}}},"pistons":{"type":"Array","typename":"vec","data":[]}}},"gearbox":{"type":"Enum","typename":"return_object::Gearbox","variant":"Automatic"}}}"#
                );
            }
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
                        6 | 8 => 3,
                        _ => 0,
                    }
                );
                assert_eq!(
                    function_name,
                    match i {
                        0 => "return_object::main",
                        1 => "return_object::create_manual_car",
                        4 => "return_object::create_auto_car",
                        6 => "return_object::choose_a_car_for_me",
                        8 => "return_object::choose_a_car_for_me",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            (
                _,
                Event::FunctionReturn {
                    function_name,
                    mut return_value,
                    ..
                },
            ) => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "return_object::create_manual_car",
                        5 => "return_object::create_auto_car",
                        7 => "return_object::choose_a_car_for_me",
                        9 => "return_object::choose_a_car_for_me",
                        10 => "return_object::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                return_value.redact_addr();
                let json = serde_json::to_string(&return_value).unwrap();
                assert_eq!(
                    json,
                    match i {
                        2 =>
                            r#"{"type":"Struct","typename":"return_object::Car","fields":{"brand":{"type":"String","typename":"&str","value":"Ford"},"engine":{"type":"Struct","typename":"return_object::Engine","fields":{"config":{"type":"Union","typeinfo":{"name":"return_object::EngineConfig","variants":["Inline","Vshape"]},"variant":"Inline","fields":{"i":{"type":"Prim","typename":"i32","value":4}}},"pistons":{"type":"Array","typename":"vec","data":[{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":1}}},{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":2}}},{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":3}}},{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":4}}}]}}},"gearbox":{"type":"Enum","typename":"return_object::Gearbox","variant":"Manual"}}}"#,
                        5 =>
                            r#"{"type":"Struct","typename":"return_object::Car","fields":{"brand":{"type":"String","typename":"&str","value":"Mazda"},"engine":{"type":"Struct","typename":"return_object::Engine","fields":{"config":{"type":"Union","typeinfo":{"name":"return_object::EngineConfig","variants":["Inline","Vshape"]},"variant":"Vshape","fields":{"0":{"type":"Prim","typename":"i16","value":3},"1":{"type":"Prim","typename":"i16","value":3}}},"pistons":{"type":"Array","typename":"vec","data":[]}}},"gearbox":{"type":"Enum","typename":"return_object::Gearbox","variant":"Automatic"}}}"#,
                        7 =>
                            r#"{"type":"Ref","typename":"ref","addr":"<redacted>","value":{"type":"Struct","typename":"return_object::Car","fields":{"brand":{"type":"String","typename":"&str","value":"Mazda"},"engine":{"type":"Struct","typename":"return_object::Engine","fields":{"config":{"type":"Union","typeinfo":{"name":"return_object::EngineConfig","variants":["Inline","Vshape"]},"variant":"Vshape","fields":{"0":{"type":"Prim","typename":"i16","value":3},"1":{"type":"Prim","typename":"i16","value":3}}},"pistons":{"type":"Array","typename":"vec","data":[]}}},"gearbox":{"type":"Enum","typename":"return_object::Gearbox","variant":"Automatic"}}}}"#,
                        9 =>
                            r#"{"type":"Ref","typename":"ref","addr":"<redacted>","value":{"type":"Struct","typename":"return_object::Car","fields":{"brand":{"type":"String","typename":"&str","value":"Ford"},"engine":{"type":"Struct","typename":"return_object::Engine","fields":{"config":{"type":"Union","typeinfo":{"name":"return_object::EngineConfig","variants":["Inline","Vshape"]},"variant":"Inline","fields":{"i":{"type":"Prim","typename":"i32","value":4}}},"pistons":{"type":"Array","typename":"vec","data":[{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":1}}},{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":2}}},{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":3}}},{"type":"Struct","typename":"return_object::Piston","fields":{"0":{"type":"Prim","typename":"u8","value":4}}}]}}},"gearbox":{"type":"Enum","typename":"return_object::Gearbox","variant":"Manual"}}}}"#,
                        10 => r#"{"type":"Unit"}"#,
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
