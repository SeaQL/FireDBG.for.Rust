mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream, PValue, RValue};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};
use std::ops::Deref;

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_ref";

    let debugger_params = debugger_params_from_file(testcase);

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..20 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);
        if matches!(i, 2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 | 19) {
            match event {
                Event::FunctionReturn { return_value, .. } => {
                    if let RValue::Ref { value, .. } = return_value {
                        match value.deref() {
                            RValue::Prim(v) => match i {
                                2 => assert_eq!(v, &PValue::bool(true)),
                                4 => assert_eq!(v, &PValue::u8(22)),
                                6 => assert_eq!(v, &PValue::i32(222_222)),
                                8 => assert_eq!(v, &PValue::i64(22_222_222_222)),
                                10 => assert_eq!(v, &PValue::i128(22_222_222_222_222_222_222)),
                                12 => assert_eq!(v, &PValue::f32(2.0)),
                                14 => assert_eq!(v, &PValue::f64(2.0)),
                                i => panic!("Unexpected i {i}"),
                            },
                            value @ RValue::Struct { .. } => {
                                let json = serde_json::to_string(value).unwrap();
                                assert_eq!(
                                    json,
                                    match i {
                                        16 =>
                                            r#"{"type":"Struct","typename":"return_ref::Small","fields":{"a":{"type":"Prim","typename":"i32","value":2},"b":{"type":"Prim","typename":"i64","value":"3"}}}"#,
                                        18 =>
                                            r#"{"type":"Struct","typename":"return_ref::Big","fields":{"a":{"type":"Prim","typename":"i32","value":4},"b":{"type":"Prim","typename":"i64","value":"3"},"c":{"type":"String","typename":"&str","value":"2"}}}"#,
                                        i => panic!("Unexpected i {i}"),
                                    }
                                );
                            }
                            _ => panic!("{value:?}"),
                        }
                    } else if i == 19 {
                        assert!(matches!(return_value, RValue::Unit));
                    } else {
                        panic!("{return_value:?}");
                    }
                }
                _ => panic!("{event:?}"),
            }
        }
    }

    Ok(())
}
