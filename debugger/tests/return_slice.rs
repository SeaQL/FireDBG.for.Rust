mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_slice";
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
            name: "u8".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8".into(),
            value: "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]".into(),
        },
        Expected::FnCall {
            name: "i8".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i8".into(),
            value: "[1i8]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1i8]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1i8]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1i8]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1i8]".into(),
        },
        Expected::FnCall {
            name: "u16".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u16".into(),
            value: "[1u16, 2u16, 3u16, 4u16, 5u16, 6u16]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1u16, 2u16, 3u16, 4u16, 5u16, 6u16]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1u16, 2u16, 3u16, 4u16, 5u16, 6u16]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1u16, 2u16, 3u16, 4u16, 5u16]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1u16, 2u16, 3u16, 4u16, 5u16]".into(),
        },
        Expected::FnCall {
            name: "i16".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i16".into(),
            value: "[1i16]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1i16]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1i16]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[]".into(),
        },
        Expected::FnCall {
            name: "u32".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u32".into(),
            value: "[1u32, 2u32, 3u32, 4u32]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1u32, 2u32, 3u32, 4u32]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1u32, 2u32, 3u32, 4u32]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[3u32]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[3u32]".into(),
        },
        Expected::FnCall {
            name: "i32".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i32".into(),
            value: "[1i32]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1i32]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1i32]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[]".into(),
        },
        Expected::FnCall {
            name: "u64".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u64".into(),
            value: "[1u64, 2u64, 3u64]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1u64, 2u64, 3u64]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1u64, 2u64, 3u64]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[2u64, 3u64]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[2u64, 3u64]".into(),
        },
        Expected::FnCall {
            name: "i64".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i64".into(),
            value: "[1i64]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1i64]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1i64]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[]".into(),
        },
        Expected::FnCall {
            name: "u128".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u128".into(),
            value: "[1u128, 2u128, 3u128]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1u128, 2u128, 3u128]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1u128, 2u128, 3u128]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[2u128, 3u128]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[2u128, 3u128]".into(),
        },
        Expected::FnCall {
            name: "i128".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i128".into(),
            value: "[1i128]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1i128]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1i128]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[]".into(),
        },
        Expected::FnCall {
            name: "f32".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f32".into(),
            value: "[1f32, 2f32, 3f32]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1f32, 2f32, 3f32]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1f32, 2f32, 3f32]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[2f32, 3f32]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[2f32, 3f32]".into(),
        },
        Expected::FnCall {
            name: "f64".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f64".into(),
            value: "[1f64]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1f64]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1f64]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[]".into(),
        },
        Expected::FnCall {
            name: "usize".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "usize".into(),
            value: "[1usize, 2usize, 3usize]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1usize, 2usize, 3usize]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1usize, 2usize, 3usize]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[2usize, 3usize]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[2usize, 3usize]".into(),
        },
        Expected::FnCall {
            name: "isize".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "isize".into(),
            value: "[1isize]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[1isize]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[1isize]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[]".into(),
        },
        Expected::FnCall {
            name: "str".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "str".into(),
            value: r#"["1", "2", "3"]"#.into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec![r#"&["1", "2", "3"]"#.into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: r#"&["1", "2", "3"]"#.into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec![r#"&["2", "3"]"#.into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: r#"&["2", "3"]"#.into(),
        },
        Expected::FnCall {
            name: "wrapped".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "wrapped".into(),
            value: "return_slice::Wrapper<[u8; 9]>([0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&return_slice::Wrapper<[u8; 9]>([0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&return_slice::Wrapper<[u8; 9]>([0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])".into(),
        },
        Expected::FnCall {
            name: "wrapped_ref".into(),
            args: vec!["&[0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]".into()],
        },
        Expected::FnRet {
            name: "wrapped_ref".into(),
            value: "return_slice::RefWrapper<[u8; 9]>(&[0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])".into(),
        },
        Expected::FnCall {
            name: "multi".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi".into(),
            value: "[[0x01, 0x02, 0x03], [0x04, 0x05, 0x06], [0x07, 0x08, 0x09]]".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[[0x01, 0x02, 0x03], [0x04, 0x05, 0x06], [0x07, 0x08, 0x09]]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[[0x01, 0x02, 0x03], [0x04, 0x05, 0x06], [0x07, 0x08, 0x09]]".into(),
        },
        Expected::FnCall {
            name: "multi_w".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_w".into(),
            value: "return_slice::Wrapper<[[u8; 3]; 3]>([[0x09, 0x08, 0x07], [0x06, 0x05, 0x04], [0x03, 0x02, 0x01]])".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&return_slice::Wrapper<[[u8; 3]; 3]>([[0x09, 0x08, 0x07], [0x06, 0x05, 0x04], [0x03, 0x02, 0x01]])".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&return_slice::Wrapper<[[u8; 3]; 3]>([[0x09, 0x08, 0x07], [0x06, 0x05, 0x04], [0x03, 0x02, 0x01]])".into(),
        },
        Expected::FnCall {
            name: "multi_ref_w".into(),
            args: vec!["&[[0x09, 0x08, 0x07], [0x06, 0x05, 0x04], [0x03, 0x02, 0x01]]".into()],
        },
        Expected::FnRet {
            name: "multi_ref_w".into(),
            value: "return_slice::RefWrapper<[[u8; 3]; 3]>(&[[0x09, 0x08, 0x07], [0x06, 0x05, 0x04], [0x03, 0x02, 0x01]])".into(),
        },
        Expected::FnCall {
            name: "borrow".into(),
            args: vec!["&[2i32]".into()],
        },
        Expected::FnRet {
            name: "borrow".into(),
            value: "&[2i32]".into(),
        },
        Expected::FnCall {
            name: "slice".into(),
            args: vec!["&[1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32]".into(), "1usize".into(), "2usize".into()],
        },
        Expected::FnRet {
            name: "slice".into(),
            value: "&[2i32]".into(),
        },
        Expected::FnCall {
            name: "slice".into(),
            args: vec!["&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]".into(), "0usize".into(), "0usize".into()],
        },
        Expected::FnRet {
            name: "slice".into(),
            value: "&[]".into(),
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
