mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    ArrayType, Breakpoint, BreakpointType, Bytes, Debugger, DebuggerParams, Event, EventStream,
    LineColumn, PValue, RValue, SourceFile, StringType, VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};
use std::time::SystemTime;

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "prim";
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
                        line: 62,
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
    let payload = consumer.next().await?.message().into_bytes();
    let event = EventStream::read_from(Bytes::from(payload));
    match event {
        Event::Breakpoint { locals, .. } => {
            assert_eq!(locals.len(), 29);
            for (i, (name, value)) in locals.iter().enumerate() {
                let json = serde_json::to_string(value).unwrap();
                println!("{name} = {json}");
                if let RValue::Prim(value) = value {
                    match (i, value) {
                        (0, PValue::bool(v)) => assert_eq!(v, &true),
                        (1, PValue::bool(v)) => assert_eq!(v, &false),
                        (2, PValue::char(v)) => assert_eq!(v, &'A'),
                        (3, PValue::u8(v)) => assert_eq!(v, &1),
                        (4, PValue::i8(v)) => assert_eq!(v, &-1),
                        (5, PValue::u16(v)) => assert_eq!(v, &2),
                        (6, PValue::i16(v)) => assert_eq!(v, &-2),
                        (7, PValue::u32(v)) => assert_eq!(v, &3),
                        (8, PValue::i32(v)) => assert_eq!(v, &-3),
                        (9, PValue::u64(v)) => assert_eq!(v, &4),
                        (10, PValue::i64(v)) => assert_eq!(v, &-4),
                        (11, PValue::usize(v)) => assert_eq!(v, &5),
                        (12, PValue::isize(v)) => assert_eq!(v, &-5),
                        (13, PValue::u128(v)) => assert_eq!(v, &6),
                        (14, PValue::i128(v)) => assert_eq!(v, &-6),
                        (15, PValue::f32(v)) => assert_eq!(v, &3.14159274),
                        (16, PValue::f64(v)) => assert_eq!(v, &3.14159265359),
                        err => panic!("{err:?}"),
                    }
                } else if i == 17 {
                    assert!(matches!(value, RValue::Unit));
                } else if i == 18 || i == 19 {
                    match value {
                        RValue::Bytes { typename, value } => {
                            match i {
                                18 => assert_eq!(typename.as_str(), "[u8; 5]"),
                                19 => assert_eq!(typename.as_str(), "&[u8]"),
                                _ => unreachable!(),
                            }
                            assert_eq!(value.as_slice(), &[1, 2, 3, 4, 5]);
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 20 {
                    match value {
                        RValue::Bytes { typename, value } => {
                            assert_eq!(typename.as_str(), "Vec<u8>");
                            assert_eq!(value.as_slice(), &[5, 6, 7, 8, 9]);
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 21 {
                    match value {
                        RValue::String {
                            typename: StringType::StrLit,
                            value,
                        } => {
                            assert_eq!(value.as_str(), "hello world");
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 22 {
                    match value {
                        RValue::String {
                            typename: StringType::String,
                            value,
                        } => {
                            assert_eq!(value.as_str(), "hello world!");
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 23 {
                    match value {
                        RValue::Bytes { typename, value } => {
                            assert_eq!(typename.as_str(), "Vec<u8>");
                            let val: Vec<u8> = Vec::new();
                            assert_eq!(value.as_slice(), &val);
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 24 {
                    match value {
                        RValue::Array {
                            typename: ArrayType::Vec,
                            data,
                        } => {
                            let val: Vec<RValue> = Vec::new();
                            assert_eq!(data, &val);
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 25 {
                    match value {
                        RValue::Bytes { typename, value } => {
                            assert_eq!(typename.as_str(), "&[u8]");
                            let val: Vec<u8> = Vec::new();
                            assert_eq!(value.as_slice(), &val);
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 26 {
                    match value {
                        RValue::Array {
                            typename: ArrayType::Slice,
                            data,
                        } => {
                            let val: Vec<RValue> = Vec::new();
                            assert_eq!(data, &val);
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 27 {
                    match value {
                        RValue::String {
                            typename: StringType::String,
                            value,
                        } => {
                            assert_eq!(value.as_str(), "");
                        }
                        err => panic!("{err:?}"),
                    }
                } else if i == 28 {
                    match value {
                        RValue::String {
                            typename: StringType::StrLit,
                            value,
                        } => {
                            assert_eq!(value.as_str(), "");
                        }
                        err => panic!("{err:?}"),
                    }
                }
            }
        }
        other => panic!("Unexpected {other:?}"),
    }

    Ok(())
}
