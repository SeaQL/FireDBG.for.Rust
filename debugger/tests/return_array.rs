mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, EventStream};
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_array";
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
            name: "u8_10".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_10".into(),
            value: "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a]".into(),
        },
        Expected::FnCall {
            name: "u8_9".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_9".into(),
            value: "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]".into(),
        },
        Expected::FnCall {
            name: "u8_8".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_8".into(),
            value: "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]".into(),
        },
        Expected::FnCall {
            name: "u8_6".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_6".into(),
            value: "[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]".into(),
        },
        Expected::FnCall {
            name: "u8_4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_4".into(),
            value: "[0x01, 0x02, 0x03, 0x04]".into(),
        },
        Expected::FnCall {
            name: "u8_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_2".into(),
            value: "[0x01, 0x02]".into(),
        },
        Expected::FnCall {
            name: "u8_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u8_1".into(),
            value: "[0x01]".into(),
        },
        Expected::FnCall {
            name: "u16_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u16_1".into(),
            value: "[1u16]".into(),
        },
        Expected::FnCall {
            name: "u16_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u16_2".into(),
            value: "[1u16, 2u16]".into(),
        },
        Expected::FnCall {
            name: "u16_4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u16_4".into(),
            value: "[1u16, 2u16, 3u16, 4u16]".into(),
        },
        Expected::FnCall {
            name: "u16_5".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u16_5".into(),
            value: "[1u16, 2u16, 3u16, 4u16, 5u16]".into(),
        },
        Expected::FnCall {
            name: "u16_6".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u16_6".into(),
            value: "[1u16, 2u16, 3u16, 4u16, 5u16, 6u16]".into(),
        },
        Expected::FnCall {
            name: "i16_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i16_1".into(),
            value: "[1i16]".into(),
        },
        Expected::FnCall {
            name: "i16_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i16_2".into(),
            value: "[1i16, 2i16]".into(),
        },
        Expected::FnCall {
            name: "i16_4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i16_4".into(),
            value: "[1i16, 2i16, 3i16, 4i16]".into(),
        },
        Expected::FnCall {
            name: "i16_5".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i16_5".into(),
            value: "[1i16, 2i16, 3i16, 4i16, 5i16]".into(),
        },
        Expected::FnCall {
            name: "i16_6".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i16_6".into(),
            value: "[1i16, 2i16, 3i16, 4i16, 5i16, 6i16]".into(),
        },
        Expected::FnCall {
            name: "u32_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u32_1".into(),
            value: "[1u32]".into(),
        },
        Expected::FnCall {
            name: "u32_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u32_2".into(),
            value: "[1u32, 2u32]".into(),
        },
        Expected::FnCall {
            name: "u32_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u32_3".into(),
            value: "[1u32, 2u32, 3u32]".into(),
        },
        Expected::FnCall {
            name: "u32_4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u32_4".into(),
            value: "[1u32, 2u32, 3u32, 4u32]".into(),
        },
        Expected::FnCall {
            name: "i32_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i32_1".into(),
            value: "[1i32]".into(),
        },
        Expected::FnCall {
            name: "i32_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i32_2".into(),
            value: "[1i32, 2i32]".into(),
        },
        Expected::FnCall {
            name: "i32_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i32_3".into(),
            value: "[1i32, 2i32, 3i32]".into(),
        },
        Expected::FnCall {
            name: "i32_4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i32_4".into(),
            value: "[1i32, 2i32, 3i32, 4i32]".into(),
        },
        Expected::FnCall {
            name: "u64_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u64_1".into(),
            value: "[1u64]".into(),
        },
        Expected::FnCall {
            name: "u64_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u64_2".into(),
            value: "[1u64, 2u64]".into(),
        },
        Expected::FnCall {
            name: "u64_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u64_3".into(),
            value: "[1u64, 2u64, 3u64]".into(),
        },
        Expected::FnCall {
            name: "i64_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i64_1".into(),
            value: "[1i64]".into(),
        },
        Expected::FnCall {
            name: "i64_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i64_2".into(),
            value: "[1i64, 2i64]".into(),
        },
        Expected::FnCall {
            name: "i64_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i64_3".into(),
            value: "[1i64, 2i64, 3i64]".into(),
        },
        Expected::FnCall {
            name: "u128_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u128_1".into(),
            value: "[1u128]".into(),
        },
        Expected::FnCall {
            name: "u128_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u128_2".into(),
            value: "[1u128, 2u128]".into(),
        },
        Expected::FnCall {
            name: "u128_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "u128_3".into(),
            value: "[1u128, 2u128, 3u128]".into(),
        },
        Expected::FnCall {
            name: "i128_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i128_1".into(),
            value: "[1i128]".into(),
        },
        Expected::FnCall {
            name: "i128_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i128_2".into(),
            value: "[1i128, 2i128]".into(),
        },
        Expected::FnCall {
            name: "i128_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "i128_3".into(),
            value: "[1i128, 2i128, 3i128]".into(),
        },
        Expected::FnCall {
            name: "f32_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f32_1".into(),
            value: "[1f32]".into(),
        },
        Expected::FnCall {
            name: "f32_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f32_2".into(),
            value: "[1f32, 2f32]".into(),
        },
        Expected::FnCall {
            name: "f32_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f32_3".into(),
            value: "[1f32, 2f32, 3f32]".into(),
        },
        Expected::FnCall {
            name: "f64_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f64_1".into(),
            value: "[1f64]".into(),
        },
        Expected::FnCall {
            name: "f64_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f64_2".into(),
            value: "[1f64, 2f64]".into(),
        },
        Expected::FnCall {
            name: "f64_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "f64_3".into(),
            value: "[1f64, 2f64, 3f64]".into(),
        },
        Expected::FnCall {
            name: "usize_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "usize_1".into(),
            value: "[1usize]".into(),
        },
        Expected::FnCall {
            name: "usize_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "usize_2".into(),
            value: "[1usize, 2usize]".into(),
        },
        Expected::FnCall {
            name: "usize_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "usize_3".into(),
            value: "[1usize, 2usize, 3usize]".into(),
        },
        Expected::FnCall {
            name: "isize_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "isize_1".into(),
            value: "[1isize]".into(),
        },
        Expected::FnCall {
            name: "isize_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "isize_2".into(),
            value: "[1isize, 2isize]".into(),
        },
        Expected::FnCall {
            name: "isize_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "isize_3".into(),
            value: "[1isize, 2isize, 3isize]".into(),
        },
        Expected::FnCall {
            name: "str_1".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "str_1".into(),
            value: r#"["1"]"#.into(),
        },
        Expected::FnCall {
            name: "str_2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "str_2".into(),
            value: r#"["1", "2"]"#.into(),
        },
        Expected::FnCall {
            name: "str_3".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "str_3".into(),
            value: r#"["1", "2", "3"]"#.into(),
        },
        Expected::FnCall {
            name: "wrapped_9".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "wrapped_9".into(),
            value: "return_array::Wrapper<[u8; 9]>([0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])"
                .into(),
        },
        Expected::FnCall {
            name: "wrapped_8".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "wrapped_8".into(),
            value: "return_array::Wrapper<[u8; 8]>([0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])"
                .into(),
        },
        Expected::FnCall {
            name: "multi_9".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_9".into(),
            value: "[[0x01, 0x02, 0x03], [0x04, 0x05, 0x06], [0x07, 0x08, 0x09]]".into(),
        },
        Expected::FnCall {
            name: "multi_4x2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_4x2".into(),
            value: "[[0x01, 0x02, 0x03, 0x04], [0x05, 0x06, 0x07, 0x08]]".into(),
        },
        Expected::FnCall {
            name: "multi_2x4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_2x4".into(),
            value: "[[0x01, 0x02], [0x03, 0x04], [0x05, 0x06], [0x07, 0x08]]".into(),
        },
        Expected::FnCall {
            name: "multi_w_9".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_w_9".into(),
            value: "return_array::Wrapper<[[u8; 3]; 3]>([[0x09, 0x08, 0x07], [0x06, 0x05, 0x04], [0x03, 0x02, 0x01]])".into(),
        },
        Expected::FnCall {
            name: "multi_w_4x2".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_w_4x2".into(),
            value: "return_array::Wrapper<[[u8; 4]; 2]>([[0x08, 0x07, 0x06, 0x05], [0x04, 0x03, 0x02, 0x01]])".into(),
        },
        Expected::FnCall {
            name: "multi_w_2x4".into(),
            args: vec![],
        },
        Expected::FnRet {
            name: "multi_w_2x4".into(),
            value: "return_array::Wrapper<[[u8; 2]; 4]>([[0x08, 0x07], [0x06, 0x05], [0x04, 0x03], [0x02, 0x01]])".into(),
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
