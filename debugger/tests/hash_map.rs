mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream, PValue, RValue};
use sea_streamer::{Buffer, Consumer, Message, Producer};
use std::fmt::Debug;

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "hash_map";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..28 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));

        match &event {
            Event::FunctionCall { arguments, .. } => match i {
                0 => (),
                1 => {
                    assert_eq!(arguments.len(), 1);
                    verify(
                        &arguments[0].1,
                        vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)],
                        ("", "i32"),
                    );
                }
                3 => {
                    assert_eq!(arguments.len(), 1);
                    verify(
                        &arguments[0].1,
                        vec![(b'a', 1), (b'b', 2), (b'c', 3), (b'd', 4)],
                        ("u8", "u8"),
                    );
                }
                5 => {
                    assert_eq!(arguments.len(), 1);
                    verify(
                        &arguments[0].1,
                        vec![
                            ("aa", 111_111),
                            ("bb", 222_222),
                            ("cc", 333_333),
                            ("dd", 444_444),
                        ],
                        ("", "u32"),
                    );
                }
                7 => {
                    assert_eq!(arguments.len(), 1);
                    verify(
                        &arguments[0].1,
                        vec![
                            ("aa", 111_111_111),
                            ("bb", 222_222_222),
                            ("cc", 333_333_333),
                            ("dd", 444_444_444),
                        ],
                        ("", "u64"),
                    );
                }
                9 => {
                    assert_eq!(arguments.len(), 1);
                    verify(
                        &arguments[0].1,
                        vec![
                            ('a', 1),
                            ('b', 2),
                            ('c', 3),
                            ('d', 4),
                            ('e', 5),
                            ('f', 6),
                            ('g', 7),
                            ('h', 8),
                            ('i', 9),
                        ],
                        ("", "i64"),
                    );
                }
                11 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[('a', hash_map::Point<i32> { x: 10i32, y: 11i32 }), ('b', hash_map::Point<i32> { x: 20i32, y: 22i32 }), ('c', hash_map::Point<i32> { x: 30i32, y: 33i32 }), ('d', hash_map::Point<i32> { x: 40i32, y: 44i32 })].into_iter().collect::<std::collections::hash::map::HashMap<char, hash_map::Point<i32>>>()"#
                    )
                }
                13 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[(97u8, hash_map::Point<i32> { x: 10i32, y: 11i32 }), (98u8, hash_map::Point<i32> { x: 20i32, y: 22i32 }), (99u8, hash_map::Point<i32> { x: 30i32, y: 33i32 }), (100u8, hash_map::Point<i32> { x: 40i32, y: 44i32 })].into_iter().collect::<std::collections::hash::map::HashMap<u8, hash_map::Point<i32>>>()"#
                    )
                }
                15 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[(97u8, hash_map::Point<f32> { x: 1f32, y: 1.1f32 }), (98u8, hash_map::Point<f32> { x: 2f32, y: 2.2f32 }), (99u8, hash_map::Point<f32> { x: 3f32, y: 3.3f32 }), (100u8, hash_map::Point<f32> { x: 4f32, y: 4.4f32 })].into_iter().collect::<std::collections::hash::map::HashMap<u8, hash_map::Point<f32>>>()"#
                    )
                }
                17 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[1u32, 2u32, 3u32, 4u32].into_iter().collect::<std::collections::hash::set::HashSet<u32>>()"#
                    )
                }
                19 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32].into_iter().collect::<std::collections::hash::set::HashSet<i32>>()"#
                    )
                }
                21 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8].into_iter().collect::<std::collections::hash::set::HashSet<u8>>()"#
                    )
                }
                23 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[(String::from("aa"), String::from("aaaa")), (String::from("bb"), String::from("bbbb")), (String::from("cc"), String::from("cccc")), (String::from("dd"), String::from("dddd"))].into_iter().collect::<std::collections::hash::map::HashMap<alloc::string::String, alloc::string::String>>()"#
                    )
                }
                25 => match &arguments[0].1 {
                    RValue::Struct { fields, .. } => match fields.get("items").unwrap() {
                        RValue::Array { data: items, .. } => {
                            assert_eq!(items.len(), 1000);
                            let sum: i32 = items
                                .iter()
                                .map(|v| match v {
                                    RValue::Prim(PValue::i32(v)) => *v,
                                    _ => panic!("Unexpected RValue"),
                                })
                                .sum();
                            assert_eq!(sum, 1000 * 1001 / 2);
                        }
                        _ => panic!("Unexpected RValue"),
                    },
                    _ => panic!("Unexpected RValue"),
                },
                27 => {
                    let pretty = format!("{}", &arguments[0].1);
                    println!("{pretty}");
                    assert_eq!(
                        pretty,
                        r#"[(String::from("aa"), vec![(String::from("aaaa"), 1i32)]), (String::from("bb"), vec![(String::from("bbbb"), 2i32)]), (String::from("cc"), vec![(String::from("cccc"), 3i32)]), (String::from("dd"), vec![(String::from("dddd"), 4i32)])].into_iter().collect::<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<(alloc::string::String, i32)>>>()"#
                    )
                }
                _ => panic!("Unexpected {i}"),
            },
            _ => (),
        }
    }

    Ok(())
}

fn verify<E: Debug, F: Debug>(rvalue: &RValue, template: Vec<(E, F)>, (at, bt): (&str, &str)) {
    let pretty = format!("{rvalue}");
    println!("{pretty}");
    for (a, b) in template {
        assert!(pretty.contains(&format!("({a:?}{at}, {b:?}{bt})")));
    }
}
