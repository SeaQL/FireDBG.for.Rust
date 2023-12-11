mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{ArgumentList, Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_char";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..18 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall {
                function_name,
                arguments,
                ..
            } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "return_char::main",
                        1 => "return_char::alpha",
                        3 => "return_char::beta",
                        5 => "return_char::charlie",
                        7 => "return_char::delta",
                        9 => "return_char::delta",
                        11 => "return_char::delta",
                        13 => "return_char::delta",
                        15 => "return_char::delta",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                println!("[{i}] {function_name} ({})", ArgumentList(arguments));
                let arguments = serde_json::to_string(&arguments).unwrap();
                assert_eq!(
                    arguments,
                    match i {
                        0 => r#"[]"#,
                        1 => r#"[]"#,
                        3 =>
                            r#"[["arr",{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"i32","value":1},{"type":"Prim","typename":"i32","value":2},{"type":"Prim","typename":"i32","value":3}]}]]"#,
                        5 =>
                            r#"[["arr",{"type":"Array","typename":"arr","data":[{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"O"},{"type":"Prim","typename":"char","value":"X"}]},{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"char","value":"O"},{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"O"}]},{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"X"}]}]}]]"#,
                        7 =>
                            r#"[["a",{"type":"Prim","typename":"bool","value":true}],["c",{"type":"Prim","typename":"char","value":"O"}]]"#,
                        9 =>
                            r#"[["a",{"type":"Prim","typename":"bool","value":false}],["c",{"type":"Prim","typename":"char","value":"X"}]]"#,
                        11 =>
                            r#"[["a",{"type":"Prim","typename":"bool","value":true}],["c",{"type":"Prim","typename":"char","value":"\u0000"}]]"#,
                        13 =>
                            r#"[["a",{"type":"Prim","typename":"bool","value":true}],["c",{"type":"Prim","typename":"char","value":"�"}]]"#,
                        15 =>
                            r#"[["a",{"type":"Prim","typename":"bool","value":true}],["c",{"type":"Prim","typename":"char","value":"􏿿"}]]"#,
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => {
                assert_eq!(
                    function_name,
                    match i {
                        2 => "return_char::alpha",
                        4 => "return_char::beta",
                        6 => "return_char::charlie",
                        8 => "return_char::delta",
                        10 => "return_char::delta",
                        12 => "return_char::delta",
                        14 => "return_char::delta",
                        16 => "return_char::delta",
                        17 => "return_char::main",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                println!("[{i}] {function_name}() -> {return_value}");
                let return_value = serde_json::to_string(&return_value).unwrap();
                assert_eq!(
                    return_value,
                    match i {
                        2 =>
                            r#"{"type":"Struct","typename":"return_char::Alpha","fields":{"c1":{"type":"Prim","typename":"char","value":"X"},"c2":{"type":"Prim","typename":"char","value":"O"}}}"#,
                        4 =>
                            r#"{"type":"Struct","typename":"return_char::Beta","fields":{"arr":{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"i32","value":1},{"type":"Prim","typename":"i32","value":2},{"type":"Prim","typename":"i32","value":3}]},"c":{"type":"Prim","typename":"char","value":"O"}}}"#,
                        6 =>
                            r#"{"type":"Struct","typename":"return_char::Charlie","fields":{"arr":{"type":"Array","typename":"arr","data":[{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"O"},{"type":"Prim","typename":"char","value":"X"}]},{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"char","value":"O"},{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"O"}]},{"type":"Array","typename":"arr","data":[{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"X"},{"type":"Prim","typename":"char","value":"X"}]}]},"c":{"type":"Prim","typename":"char","value":"X"}}}"#,
                        8 =>
                            r#"{"type":"Option","typename":"core::option::Option<char>","variant":"Some","value":{"type":"Prim","typename":"char","value":"O"}}"#,
                        10 =>
                            r#"{"type":"Option","typename":"core::option::Option<char>","variant":"None","value":null}"#,
                        12 =>
                            r#"{"type":"Option","typename":"core::option::Option<char>","variant":"Some","value":{"type":"Prim","typename":"char","value":"\u0000"}}"#,
                        14 =>
                            r#"{"type":"Option","typename":"core::option::Option<char>","variant":"Some","value":{"type":"Prim","typename":"char","value":"�"}}"#,
                        16 =>
                            r#"{"type":"Option","typename":"core::option::Option<char>","variant":"Some","value":{"type":"Prim","typename":"char","value":"􏿿"}}"#,
                        17 => r#"{"type":"Unit"}"#,
                        _ => panic!("Unexpected i {i}"),
                    }
                );
            }
        }
    }

    Ok(())
}
