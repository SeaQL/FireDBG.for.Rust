mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "more_result";
    let (producer, consumer) = setup(testcase).await?;

    let tuple_ab = vec!["11i32".to_owned(), "12i32".to_owned()];
    let tuple_234 = "(2i32, 3i32, 4i64)".to_owned();
    let valid_str = "[0xf0, 0x9f, 0x92, 0x96]".to_owned();
    let invalid_str = "[0x00, 0x9f, 0x92, 0x96]".to_owned();
    let debugger_params = generate_rust_program(
        testcase,
        r#"
        fn res_1(a: i32, b: i32) -> (i32, i32, i64) {
            {tuple_234}
        }
        fn open_file_1() -> std::io::Result<()> {
            std::fs::File::open("/non-existent-file")?;
            Ok(())
        }
        fn open_file_2() -> std::io::Result<std::fs::File> {
            std::fs::File::open("/dev/urandom")
        }
        fn open_file_3() -> std::io::Result<std::fs::File> {
            std::fs::File::open("/non-existent-file")
        }
        fn str_from_utf8(b: &[u8]) -> Result<&str, std::str::Utf8Error> {
            std::str::from_utf8(b)
        }
        fn string_from_utf8(b: Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
            String::from_utf8(b)
        }
        fn result_boxed(i: i32) -> Result<Box<String>, ()> {
            if i == 0 {
                Ok(Box::new(String::from("hello")))
            } else {
                Err(())
            }
        }
        fn passthru<T>(v: T) -> T {
            std::hint::black_box(v)
        }
        struct RNode {
            i: i32,
            next: Result<Box<RNode>, ()>,
        }
        fn main() {
            println!("hello");
            res_1({tuple_ab});
            assert!(open_file_1().is_err());
            assert!(open_file_2().is_ok());
            assert!(open_file_3().is_err());
            assert!(str_from_utf8(&{valid_str}).is_ok());
            assert!(str_from_utf8(&{invalid_str}).is_err());
            assert!(string_from_utf8(vec!{valid_str}).is_ok());
            assert!(string_from_utf8(vec!{invalid_str}).is_err());
            assert!(passthru(result_boxed(0)).is_ok());
            assert!(passthru(result_boxed(1)).is_err());

            let node = RNode {
                i: 1,
                next: Ok(Box::new(RNode {
                    i: 2,
                    next: Ok(Box::new(RNode {
                        i: 3,
                        next: Err(()),
                    })),
                }))
            };
            passthru(&node);
        }
        "#
        .replace("{tuple_234}", &tuple_234)
        .replace("{tuple_ab}", &tuple_ab.join(", "))
        .replace("{valid_str}", &valid_str)
        .replace("{invalid_str}", &invalid_str)
        .as_str(),
    );
    let result_boxed_ok = r#"core::result::Result::<alloc::boxed::Box<alloc::string::String>, ()>::Ok(alloc::boxed::Box::new(String::from("hello")))"#.to_owned();
    let result_boxed_err =
        r#"core::result::Result::<alloc::boxed::Box<alloc::string::String>, ()>::Err(())"#
            .to_owned();
    let result_chain =
        expand("&RNode { i: 1i32, next: Result<Box<RNode>, ()>::Ok(Box::new(RNode { i: 2i32, next: Result<Box<RNode>, ()>::Ok(Box::new(RNode { i: 3i32, next: Result<Box<RNode>, ()>::Err(()) })) })) }");
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let expected = vec![
        Expected::FnCall {
            name: "main".into(),
            args: vec![],
        },
        Expected::FnCall {
            name: "res_1".into(),
            args: tuple_ab,
        },
        Expected::FnRet {
            name: "res_1".into(),
            value: tuple_234,
        },
        Expected::FnCall {
            name: "open_file_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "open_file_1".into(),
            value:
                "core::result::Result::<(), std::io::error::Error>::Err(std::io::error::Error { code: 2i32, kind: std::io::error::ErrorKind::NotFound, .. })"
                    .into(),
        },
        Expected::FnCall {
            name: "open_file_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "open_file_2".into(),
            value: "core::result::Result::<std::fs::File, std::io::error::Error>::Ok(std::fs::File {..})"
                .into(),
        },
        Expected::FnCall {
            name: "open_file_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "open_file_3".into(),
            value: "core::result::Result::<std::fs::File, std::io::error::Error>::Err(std::io::error::Error {..})"
                .into(),
        },
        Expected::FnCall {
            name: "str_from_utf8".into(),
            args: vec![format!("&{valid_str}")],
        },
        Expected::FnRet {
            name: "str_from_utf8".into(),
            value: "core::result::Result::<&str, core::str::error::Utf8Error>::Ok(\"💖\")"
                .into(),
        },
        Expected::FnCall {
            name: "str_from_utf8".into(),
            args: vec![format!("&{invalid_str}")],
        },
        Expected::FnRet {
            name: "str_from_utf8".into(),
            value: "core::result::Result::<&str, core::str::error::Utf8Error>::Err(core::str::error::Utf8Error {..})"
                .into(),
        },
        Expected::FnCall {
            name: "string_from_utf8".into(),
            args: vec![valid_str],
        },
        Expected::FnRet {
            name: "string_from_utf8".into(),
            value: "core::result::Result::<alloc::string::String, alloc::string::FromUtf8Error>::Ok(String::from(\"💖\"))"
                .into(),
        },
        Expected::FnCall {
            name: "string_from_utf8".into(),
            args: vec![invalid_str],
        },
        Expected::FnRet {
            name: "string_from_utf8".into(),
            value: "core::result::Result::<alloc::string::String, alloc::string::FromUtf8Error>::Err(alloc::string::FromUtf8Error { bytes: [..], error: core::str::error::Utf8Error { valid_up_to: 1usize, error_len: core::option::Option::<u8>::Some(1u8) } })"
                .into(),
        },
        Expected::FnCall {
            name: "result_boxed".into(),
            args: vec!["0i32".into()],
        },
        Expected::FnRet {
            name: "result_boxed".into(),
            value: result_boxed_ok.clone(),
        },
        Expected::FnCall {
            name: "passthru".into(),
            args: vec![result_boxed_ok.clone()],
        },
        Expected::FnRet {
            name: "passthru".into(),
            value: result_boxed_ok.clone(),
        },
        Expected::FnCall {
            name: "result_boxed".into(),
            args: vec!["1i32".into()],
        },
        Expected::FnRet {
            name: "result_boxed".into(),
            value: result_boxed_err.clone(),
        },
        Expected::FnCall {
            name: "passthru".into(),
            args: vec![result_boxed_err.clone()],
        },
        Expected::FnRet {
            name: "passthru".into(),
            value: result_boxed_err.clone(),
        },
        Expected::FnCall {
            name: "passthru".into(),
            args: vec![result_chain.clone()],
        },
        Expected::FnRet {
            name: "passthru".into(),
            value: result_chain.clone(),
        },
        Expected::FnRet {
            name: "main".into(),
            value: "()".into(),
        },
    ];

    let mut events = Vec::new();
    for i in 0..expected.len() {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);
        events.push(event);
    }

    verify(testcase, events, expected);

    Ok(())
}

fn expand(string: &str) -> String {
    string
        .replace("RNode", "more_result::RNode")
        .replace("Result<", "core::result::Result::<")
        .replace("Box", "alloc::boxed::Box")
        .replace("Rc", "alloc::rc::Rc")
        .replace("Arc", "alloc::sync::Arc")
}
