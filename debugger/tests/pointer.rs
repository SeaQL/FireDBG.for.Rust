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
    let testcase = "pointer";
    let (producer, consumer) = setup(testcase).await?;

    rustc(&format!("testcases/{testcase}"));

    Debugger::run(
        DebuggerParams {
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
                        line: 30,
                        column: None,
                    },
                    loc_end: None,
                    breakpoint_type: BreakpointType::Breakpoint,
                    capture: VariableCapture::Locals,
                },
                Breakpoint {
                    id: 2,
                    file_id: 1,
                    loc: LineColumn {
                        line: 44,
                        column: None,
                    },
                    loc_end: None,
                    breakpoint_type: BreakpointType::Breakpoint,
                    capture: VariableCapture::Locals,
                },
            ],
            arguments: vec![],
        },
        producer.clone(),
    );

    producer.end().await?;
    for i in 0..2 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        match event {
            Event::Breakpoint { mut locals, .. } => match i {
                0 => {
                    assert!(locals.len() >= 3);
                    for (j, (name, value)) in locals.iter_mut().enumerate().take(3) {
                        value.redact_addr();
                        let json = serde_json::to_string(value).unwrap();
                        println!("{name} = {json}");
                        match j {
                            0 => {
                                assert_eq!(name.as_str(), "boxed");
                                assert_eq!(
                                    json,
                                    r#"{"type":"Ref","typename":"Box","addr":"<redacted>","value":{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Boxed"}}}}"#
                                )
                            }
                            1 => {
                                assert_eq!(name.as_str(), "arc");
                                assert_eq!(
                                    json,
                                    r#"{"type":"RefCounted","typename":"Arc","addr":"<redacted>","strong":2,"weak":3,"value":{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Arced"}}}}"#
                                )
                            }
                            2 => {
                                assert_eq!(name.as_str(), "rc");
                                assert_eq!(
                                    json,
                                    r#"{"type":"RefCounted","typename":"Rc","addr":"<redacted>","strong":3,"weak":2,"value":{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Rced"}}}}"#
                                )
                            }
                            _ => (),
                        }
                    }
                }
                1 => {
                    assert!(locals.len() >= 5);
                    for (j, (name, value)) in locals.iter_mut().enumerate() {
                        value.redact_addr();
                        let json = serde_json::to_string(value).unwrap();
                        println!("{name} = {json}");
                        match j {
                            0 => {
                                assert_eq!(name.as_str(), "obj");
                                assert_eq!(
                                    json,
                                    r#"{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Don't care"}}}"#
                                )
                            }
                            1 => {
                                assert_eq!(name.as_str(), "boxed");
                                assert_eq!(
                                    json,
                                    r#"{"type":"DynRef","typename":"alloc::boxed::Box<dyn pointer::Shape>","addr":"<redacted>","vtable":"<redacted>","value":{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Don't care"}}}}"#
                                )
                            }
                            2 => {
                                assert_eq!(name.as_str(), "arc");
                                assert_eq!(
                                    json,
                                    r#"{"type":"DynRefCounted","typename":"alloc::sync::Arc<dyn pointer::Shape>","addr":"<redacted>","strong":1,"weak":1,"vtable":"<redacted>","value":{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Don't care"}}}}"#
                                )
                            }
                            3 => {
                                assert_eq!(name.as_str(), "rc");
                                assert_eq!(
                                    json,
                                    r#"{"type":"DynRefCounted","typename":"alloc::rc::Rc<dyn pointer::Shape>","addr":"<redacted>","strong":1,"weak":1,"vtable":"<redacted>","value":{"type":"Struct","typename":"pointer::Object","fields":{"name":{"type":"String","typename":"String","value":"Don't care"}}}}"#
                                )
                            }
                            4 => {
                                assert_eq!(name.as_str(), "obj");
                                assert_eq!(
                                    json,
                                    r#"{"type":"DynRef","typename":"&dyn pointer::Shape","addr":"<redacted>","vtable":"<redacted>","value":{"type":"Opaque"}}"#
                                )
                            }
                            _ => (),
                        }
                    }
                }
                _ => unreachable!(),
            },
            other => panic!("Unexpected {other:?}"),
        }
    }

    Ok(())
}
