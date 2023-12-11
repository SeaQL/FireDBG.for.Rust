mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "more_option_2";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let expected = vec![
        Expected::FnCall {
            name: "main".into(),
            args: vec![],
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "1.1f64".into(),
                "core::option::Option::<()>::Some(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<()>::Some(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["1.2f64".into(), "core::option::Option::<()>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<()>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "2.1f64".into(),
                "core::option::Option::<bool>::Some(false)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<bool>::Some(false)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "2.2f64".into(),
                "core::option::Option::<bool>::Some(true)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<bool>::Some(true)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["2.3f64".into(), "core::option::Option::<bool>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<bool>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "3.1f64".into(),
                "core::option::Option::<i8>::Some(-22i8)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i8>::Some(-22i8)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "3.2f64".into(),
                "core::option::Option::<i8>::Some(22i8)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i8>::Some(22i8)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["3.3f64".into(), "core::option::Option::<i8>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i8>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "4.1f64".into(),
                "core::option::Option::<u8>::Some(250u8)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u8>::Some(250u8)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["4.2f64".into(), "core::option::Option::<u8>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u8>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "5.1f64".into(),
                "core::option::Option::<i16>::Some(-22222i16)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i16>::Some(-22222i16)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "5.2f64".into(),
                "core::option::Option::<i16>::Some(22222i16)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i16>::Some(22222i16)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["5.3f64".into(), "core::option::Option::<i16>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i16>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "6.1f64".into(),
                "core::option::Option::<u16>::Some(65432u16)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u16>::Some(65432u16)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["6.2f64".into(), "core::option::Option::<u16>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u16>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "7.1f64".into(),
                "core::option::Option::<i32>::Some(-222_222i32)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i32>::Some(-222_222i32)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "7.2f64".into(),
                "core::option::Option::<i32>::Some(222_222i32)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i32>::Some(222_222i32)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["7.3f64".into(), "core::option::Option::<i32>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i32>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "8.1f64".into(),
                "core::option::Option::<u32>::Some(432_432u32)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u32>::Some(432_432u32)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["8.2f64".into(), "core::option::Option::<u32>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u32>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "9.1f64".into(),
                "core::option::Option::<i64>::Some(-22_222_222_222i64)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i64>::Some(-22_222_222_222i64)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "9.2f64".into(),
                "core::option::Option::<i64>::Some(22_222_222_222i64)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i64>::Some(22_222_222_222i64)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["9.3f64".into(), "core::option::Option::<i64>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i64>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "10.1f64".into(),
                "core::option::Option::<u64>::Some(23_232_232_232u64)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u64>::Some(23_232_232_232u64)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["10.2f64".into(), "core::option::Option::<u64>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u64>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "11.1f64".into(),
                "core::option::Option::<isize>::Some(-22_222_222_222isize)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<isize>::Some(-22_222_222_222isize)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "11.2f64".into(),
                "core::option::Option::<isize>::Some(22_222_222_222isize)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<isize>::Some(22_222_222_222isize)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "11.3f64".into(),
                "core::option::Option::<isize>::None".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<isize>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "12.1f64".into(),
                "core::option::Option::<usize>::Some(23_232_232_232usize)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<usize>::Some(23_232_232_232usize)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "12.2f64".into(),
                "core::option::Option::<usize>::None".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<usize>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "13.1f64".into(),
                "core::option::Option::<i128>::Some(-22_222_222_222_222_222_222i128)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i128>::Some(-22_222_222_222_222_222_222i128)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "13.2f64".into(),
                "core::option::Option::<i128>::Some(22_222_222_222_222_222_222i128)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i128>::Some(22_222_222_222_222_222_222i128)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "13.3f64".into(),
                "core::option::Option::<i128>::None".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<i128>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "14.1f64".into(),
                "core::option::Option::<u128>::Some(33_333_333_333_333_333_333u128)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u128>::Some(33_333_333_333_333_333_333u128)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "14.2f64".into(),
                "core::option::Option::<u128>::None".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<u128>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "15.1f64".into(),
                "core::option::Option::<f32>::Some(111.111f32)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<f32>::Some(111.111f32)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "15.2f64".into(),
                "core::option::Option::<f32>::Some(-111.111f32)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<f32>::Some(-111.111f32)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["15.3f64".into(), "core::option::Option::<f32>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<f32>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "16.1f64".into(),
                "core::option::Option::<f64>::Some(222.222f64)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<f64>::Some(222.222f64)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "16.2f64".into(),
                "core::option::Option::<f64>::Some(-222.222f64)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<f64>::Some(-222.222f64)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec!["16.3f64".into(), "core::option::Option::<f64>::None".into()],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<f64>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "17.1f64".into(),
                r#"core::option::Option::<&str>::Some("hello")"#.into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: r#"core::option::Option::<&str>::Some("hello")"#.into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "17.2f64".into(),
                "core::option::Option::<&str>::None".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<&str>::None".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "18.1f64".into(),
                "core::option::Option::<char>::Some('🦀')".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<char>::Some('🦀')".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "18.2f64".into(),
                "core::option::Option::<char>::None".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::option::Option::<char>::None".into(),
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
