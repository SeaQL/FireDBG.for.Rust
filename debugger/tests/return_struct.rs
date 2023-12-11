mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_struct";

    let debugger_params = debugger_params_from_file(testcase);

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..20 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);

        match event {
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => {
                let json = serde_json::to_string(&return_value).unwrap();
                assert_eq!(
                    json,
                    match i {
                        2 =>
                            r#"{"type":"Struct","typename":"return_struct::Point","fields":{"x":{"type":"Prim","typename":"i32","value":1},"y":{"type":"Prim","typename":"i32","value":2}}}"#,
                        4 =>
                            r#"{"type":"Tuple","typename":"(return_struct::Point, i32)","items":[{"type":"Struct","typename":"return_struct::Point","fields":{"x":{"type":"Prim","typename":"i32","value":1},"y":{"type":"Prim","typename":"i32","value":2}}},{"type":"Prim","typename":"i32","value":3}]}"#,
                        6 =>
                            r#"{"type":"Struct","typename":"return_struct::Vector","fields":{"x":{"type":"Prim","typename":"f64","value":1.1},"y":{"type":"Prim","typename":"f64","value":2.1}}}"#,
                        8 =>
                            r#"{"type":"Struct","typename":"return_struct::Mixed","fields":{"x":{"type":"Prim","typename":"i32","value":4},"y":{"type":"Prim","typename":"f64","value":0.1}}}"#,
                        10 =>
                            r#"{"type":"Struct","typename":"return_struct::Wrapper<return_struct::Point>","fields":{"x":{"type":"Prim","typename":"i32","value":3},"y":{"type":"Prim","typename":"i32","value":4}}}"#,
                        12 =>
                            r#"{"type":"Struct","typename":"return_struct::Coeff","fields":{"0":{"type":"Prim","typename":"f32","value":1.1},"1":{"type":"Prim","typename":"f64","value":2.2}}}"#,
                        14 =>
                            r#"{"type":"Struct","typename":"return_struct::MapPoint","fields":{"u":{"type":"Prim","typename":"i64","value":"-22"},"v":{"type":"Prim","typename":"i64","value":"44"}}}"#,
                        16 =>
                            r#"{"type":"Struct","typename":"return_struct::Label","fields":{"s":{"type":"String","typename":"&str","value":"hello"}}}"#,
                        18 =>
                            r#"{"type":"Struct","typename":"return_struct::Long","fields":{"0":{"type":"Prim","typename":"i128","value":"22222222222222222222"}}}"#,
                        19 => r#"{"type":"Unit"}"#,
                        i => panic!("Unexpected i {i}"),
                    }
                );
                println!("[{i}] {function_name}() -> {return_value}");
            }
            _ => (),
        }
    }

    Ok(())
}
