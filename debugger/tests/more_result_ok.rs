mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "more_result_ok";
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
                "core::result::Result::<(), ()>::Ok(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<(), ()>::Ok(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "1.2f64".into(),
                "core::result::Result::<(), ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<(), ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "2.1f64".into(),
                "core::result::Result::<bool, ()>::Ok(false)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<bool, ()>::Ok(false)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "2.2f64".into(),
                "core::result::Result::<bool, ()>::Ok(true)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<bool, ()>::Ok(true)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "2.3f64".into(),
                "core::result::Result::<bool, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<bool, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "3.1f64".into(),
                "core::result::Result::<i8, ()>::Ok(-22i8)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i8, ()>::Ok(-22i8)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "3.2f64".into(),
                "core::result::Result::<i8, ()>::Ok(22i8)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i8, ()>::Ok(22i8)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "3.3f64".into(),
                "core::result::Result::<i8, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i8, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "4.1f64".into(),
                "core::result::Result::<u8, ()>::Ok(250u8)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u8, ()>::Ok(250u8)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "4.2f64".into(),
                "core::result::Result::<u8, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u8, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "5.1f64".into(),
                "core::result::Result::<i16, ()>::Ok(-22222i16)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i16, ()>::Ok(-22222i16)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "5.2f64".into(),
                "core::result::Result::<i16, ()>::Ok(22222i16)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i16, ()>::Ok(22222i16)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "5.3f64".into(),
                "core::result::Result::<i16, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i16, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "6.1f64".into(),
                "core::result::Result::<u16, ()>::Ok(65432u16)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u16, ()>::Ok(65432u16)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "6.2f64".into(),
                "core::result::Result::<u16, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u16, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "7.1f64".into(),
                "core::result::Result::<i32, ()>::Ok(-222_222i32)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i32, ()>::Ok(-222_222i32)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "7.2f64".into(),
                "core::result::Result::<i32, ()>::Ok(222_222i32)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i32, ()>::Ok(222_222i32)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "7.3f64".into(),
                "core::result::Result::<i32, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i32, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "8.1f64".into(),
                "core::result::Result::<u32, ()>::Ok(432_432u32)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u32, ()>::Ok(432_432u32)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "8.2f64".into(),
                "core::result::Result::<u32, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u32, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "9.1f64".into(),
                "core::result::Result::<i64, ()>::Ok(-22_222_222_222i64)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i64, ()>::Ok(-22_222_222_222i64)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "9.2f64".into(),
                "core::result::Result::<i64, ()>::Ok(22_222_222_222i64)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i64, ()>::Ok(22_222_222_222i64)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "9.3f64".into(),
                "core::result::Result::<i64, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i64, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "10.1f64".into(),
                "core::result::Result::<u64, ()>::Ok(23_232_232_232u64)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u64, ()>::Ok(23_232_232_232u64)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "10.2f64".into(),
                "core::result::Result::<u64, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u64, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "11.1f64".into(),
                "core::result::Result::<isize, ()>::Ok(-22_222_222_222isize)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<isize, ()>::Ok(-22_222_222_222isize)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "11.2f64".into(),
                "core::result::Result::<isize, ()>::Ok(22_222_222_222isize)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<isize, ()>::Ok(22_222_222_222isize)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "11.3f64".into(),
                "core::result::Result::<isize, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<isize, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "12.1f64".into(),
                "core::result::Result::<usize, ()>::Ok(23_232_232_232usize)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<usize, ()>::Ok(23_232_232_232usize)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "12.2f64".into(),
                "core::result::Result::<usize, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<usize, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "13.1f64".into(),
                "core::result::Result::<i128, ()>::Ok(-22_222_222_222_222_222_222i128)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i128, ()>::Ok(-22_222_222_222_222_222_222i128)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "13.2f64".into(),
                "core::result::Result::<i128, ()>::Ok(22_222_222_222_222_222_222i128)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i128, ()>::Ok(22_222_222_222_222_222_222i128)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "13.3f64".into(),
                "core::result::Result::<i128, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<i128, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "14.1f64".into(),
                "core::result::Result::<u128, ()>::Ok(33_333_333_333_333_333_333u128)"
                    .replace('_', "")
                    .into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u128, ()>::Ok(33_333_333_333_333_333_333u128)"
                .replace('_', "")
                .into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "14.2f64".into(),
                "core::result::Result::<u128, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<u128, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "15.1f64".into(),
                "core::result::Result::<f32, ()>::Ok(111.111f32)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<f32, ()>::Ok(111.111f32)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "15.2f64".into(),
                "core::result::Result::<f32, ()>::Ok(-111.111f32)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<f32, ()>::Ok(-111.111f32)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "15.3f64".into(),
                "core::result::Result::<f32, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<f32, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "16.1f64".into(),
                "core::result::Result::<f64, ()>::Ok(222.222f64)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<f64, ()>::Ok(222.222f64)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "16.2f64".into(),
                "core::result::Result::<f64, ()>::Ok(-222.222f64)".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<f64, ()>::Ok(-222.222f64)".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "16.3f64".into(),
                "core::result::Result::<f64, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<f64, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "17.1f64".into(),
                r#"core::result::Result::<&str, ()>::Ok("hello")"#.into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: r#"core::result::Result::<&str, ()>::Ok("hello")"#.into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "17.2f64".into(),
                "core::result::Result::<&str, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<&str, ()>::Err(())".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "18.1f64".into(),
                "core::result::Result::<char, ()>::Ok('🦀')".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<char, ()>::Ok('🦀')".into(),
        },
        Expected::FnCall {
            name: "capture".into(),
            args: vec![
                "18.2f64".into(),
                "core::result::Result::<char, ()>::Err(())".into(),
            ],
        },
        Expected::FnRet {
            name: "capture".into(),
            value: "core::result::Result::<char, ()>::Err(())".into(),
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
