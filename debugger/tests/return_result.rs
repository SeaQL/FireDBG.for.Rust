mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream, RValue};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_result";

    let debugger_params = debugger_params_from_file(testcase);

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..138 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);
        match event {
            Event::FunctionReturn {
                function_name,
                mut return_value,
                ..
            } => {
                return_value.redact_addr();
                let json = serde_json::to_string(&return_value).unwrap();
                if let RValue::Result { .. } = return_value {
                    assert_eq!(
                        json,
                        match i {
                            2 => r#"{"type":"Result","typename":"core::result::Result<(), ()>","variant":"Ok","value":{"type":"Unit"}}"#.to_owned(),
                            4 => r#"{"type":"Result","typename":"core::result::Result<(), ()>","variant":"Err","value":{"type":"Unit"}}"#.to_owned(),
                            6 => make_result("i8", "()", Ok(()), "8"),
                            8 => r#"{"type":"Result","typename":"core::result::Result<i8, ()>","variant":"Err","value":{"type":"Unit"}}"#.to_owned(),
                            10 => make_result("i32", "i32", Ok(()), "88888"),
                            12 => make_result("i32", "i32", Err(()), "-222222"),
                            14 => make_result("u32", "i32", Ok(()), "88888"),
                            16 => make_result("u32", "i32", Err(()), "-222222"),
                            18 => make_result("i8", "u8", Ok(()), "-2"),
                            20 => make_result("i8", "u8", Err(()), "250"),
                            22 => make_result("i16", "i32", Ok(()), "-4444"),
                            24 => make_result("i16", "i32", Err(()), "222222"),
                            26 => make_result("i32", "i64", Ok(()), "-222222"),
                            28 => make_result("i32", "i64", Err(()), "\"22222222222\""),
                            30 => make_result("i8", "i8", Ok(()), "22"),
                            32 => make_result("i8", "i8", Err(()), "-108"),
                            34 => make_result("u64", "i64", Ok(()), "\"22222222222\""),
                            36 => make_result("u64", "i64", Err(()), "\"-222222222\""),
                            38 => make_result("f32", "f32", Ok(()), "2.2"),
                            40 => make_result("f32", "f32", Err(()), "3.3"),
                            42 => make_result("f32", "f64", Ok(()), "2.2"),
                            44 => make_result("f32", "f64", Err(()), "3.3"),
                            46 => make_result("f64", "f64", Ok(()), "2.2"),
                            48 => make_result("f64", "f64", Err(()), "3.3"),
                            //
                            50 => r#"{"type":"Result","typename":"core::result::Result<&i32, &u64>","variant":"Err","value":{"type":"Ref","typename":"ref","addr":"<redacted>","value":{"type":"Prim","typename":"u64","value":"12"}}}"#.to_owned(),
                            52 => r#"{"type":"Result","typename":"core::result::Result<&i32, &u64>","variant":"Ok","value":{"type":"Ref","typename":"ref","addr":"<redacted>","value":{"type":"Prim","typename":"i32","value":2222}}}"#.to_owned(),
                            54 => // Small has been boiled down
                                r#"{"type":"Result","typename":"core::result::Result<return_result::Small, i32>","variant":"Ok","value":{"type":"Prim","typename":"i32","value":8888}}"#.to_owned(),
                            56 => make_result("return_result::Small", "i32", Err(()), "-222222"),
                            58 => r#"{"type":"Result","typename":"core::result::Result<return_result::Small, return_result::Big>","variant":"Ok","value":{"type":"Struct","typename":"return_result::Small","fields":{"i":{"type":"Struct","typename":"return_result::Inner","fields":{"0":{"type":"Prim","typename":"i32","value":8888}}}}}}"#.to_owned(),
                            60 => r#"{"type":"Result","typename":"core::result::Result<return_result::Small, return_result::Big>","variant":"Err","value":{"type":"Struct","typename":"return_result::Big","fields":{"i":{"type":"Struct","typename":"return_result::Inner","fields":{"0":{"type":"Prim","typename":"i32","value":2222}}},"t":{"type":"Prim","typename":"i64","value":"-101"}}}}"#.to_owned(),
                            62 => r#"{"type":"Result","typename":"core::result::Result<(), &str>","variant":"Ok","value":{"type":"Unit"}}"#.to_owned(),
                            64 => r#"{"type":"Result","typename":"core::result::Result<(), &str>","variant":"Err","value":{"type":"String","typename":"&str","value":"hello"}}"#.to_owned(),
                            66 => r#"{"type":"Result","typename":"core::result::Result<&str, ()>","variant":"Ok","value":{"type":"String","typename":"&str","value":"world"}}"#.to_owned(),
                            68 => r#"{"type":"Result","typename":"core::result::Result<&str, ()>","variant":"Err","value":{"type":"Unit"}}"#.to_owned(),
                            70 => r#"{"type":"Result","typename":"core::result::Result<(), bool>","variant":"Ok","value":{"type":"Unit"}}"#.to_owned(),
                            72 => make_result("()", "bool", Err(()), "true"),
                            74 => make_result("()", "bool", Err(()), "false"),
                            76 => make_result("bool", "()", Err(()), "()"),
                            78 => make_result("bool", "()", Ok(()), "true"),
                            80 => make_result("bool", "()", Ok(()), "false"),
                            82 => make_result("bool", "bool", Ok(()), "false"),
                            84 => make_result("bool", "bool", Ok(()), "true"),
                            86 => make_result("bool", "bool", Err(()), "false"),
                            88 => make_result("bool", "bool", Err(()), "true"),
                            90 => make_result("i128", "i128", Ok(()), "\"22222222222222222222\""),
                            92 => make_result("i128", "i128", Err(()), "\"-22222222222222222222\""),
                            94 => make_result("()", "i128", Ok(()), "()"),
                            96 => make_result("()", "i128", Err(()), "\"-22222222222222222222\""),
                            98 => make_result("u128", "()", Ok(()), "\"22222222222222222222\""),
                            100 => make_result("u128", "()", Err(()), "()"),
                            102 => make_result("i128", "u128", Ok(()), "\"-22222222222222222222\""),
                            104 => make_result("i128", "u128", Err(()), "\"170141183460469231731687303715884105727\""),
                            106 => r#"{"type":"Result","typename":"core::result::Result<alloc::boxed::Box<dyn return_result::Mass>, ()>","variant":"Ok","value":{"type":"DynRef","typename":"alloc::boxed::Box<dyn return_result::Mass>","addr":"<redacted>","vtable":"<redacted>","value":{"type":"Opaque"}}}"#.to_owned(),
                            108 => r#"{"type":"Result","typename":"core::result::Result<alloc::boxed::Box<dyn return_result::Mass>, ()>","variant":"Err","value":{"type":"Unit"}}"#.to_owned(),
                            110 => r#"{"type":"Result","typename":"core::result::Result<alloc::boxed::Box<dyn return_result::Mass>, alloc::boxed::Box<dyn return_result::Mass>>","variant":"Ok","value":{"type":"DynRef","typename":"alloc::boxed::Box<dyn return_result::Mass>","addr":"<redacted>","vtable":"<redacted>","value":{"type":"Struct","typename":"return_result::Big","fields":{"i":{"type":"Struct","typename":"return_result::Inner","fields":{"0":{"type":"Prim","typename":"i32","value":2222}}},"t":{"type":"Prim","typename":"i64","value":"-101"}}}}}"#.to_owned(),
                            112 => r#"{"type":"Result","typename":"core::result::Result<alloc::boxed::Box<dyn return_result::Mass>, alloc::boxed::Box<dyn return_result::Mass>>","variant":"Err","value":{"type":"DynRef","typename":"alloc::boxed::Box<dyn return_result::Mass>","addr":"<redacted>","vtable":"<redacted>","value":{"type":"Struct","typename":"return_result::Small","fields":{"i":{"type":"Struct","typename":"return_result::Inner","fields":{"0":{"type":"Prim","typename":"i32","value":8888}}}}}}}"#.to_owned(),
                            114 => r#"{"type":"Result","typename":"core::result::Result<alloc::rc::Rc<dyn return_result::Mass>, ()>","variant":"Ok","value":{"type":"DynRefCounted","typename":"alloc::rc::Rc<dyn return_result::Mass>","addr":"<redacted>","strong":1,"weak":1,"vtable":"<redacted>","value":{"type":"Struct","typename":"dyn return_result::Mass","fields":{}}}}"#.to_owned(),
                            116 => r#"{"type":"Result","typename":"core::result::Result<alloc::rc::Rc<dyn return_result::Mass>, ()>","variant":"Err","value":{"type":"Unit"}}"#.to_owned(),
                            118 => r#"{"type":"Result","typename":"core::result::Result<alloc::rc::Rc<dyn return_result::Mass>, alloc::rc::Rc<dyn return_result::Mass>>","variant":"Ok","value":{"type":"DynRefCounted","typename":"alloc::rc::Rc<dyn return_result::Mass>","addr":"<redacted>","strong":1,"weak":1,"vtable":"<redacted>","value":{"type":"Struct","typename":"return_result::Big","fields":{"i":{"type":"Struct","typename":"return_result::Inner","fields":{"0":{"type":"Prim","typename":"i32","value":2222}}},"t":{"type":"Prim","typename":"i64","value":"-101"}}}}}"#.to_owned(),
                            120 => r#"{"type":"Result","typename":"core::result::Result<alloc::rc::Rc<dyn return_result::Mass>, alloc::rc::Rc<dyn return_result::Mass>>","variant":"Err","value":{"type":"DynRefCounted","typename":"alloc::rc::Rc<dyn return_result::Mass>","addr":"<redacted>","strong":1,"weak":1,"vtable":"<redacted>","value":{"type":"Struct","typename":"return_result::Small","fields":{"i":{"type":"Struct","typename":"return_result::Inner","fields":{"0":{"type":"Prim","typename":"i32","value":8888}}}}}}}"#.to_owned(),
                            122 => make_result("char", "()", Ok(()), "\"🔥\""),
                            124 => make_result("char", "()", Err(()), "()"),
                            126 => make_result("()", "char", Ok(()), "()"),
                            128 => make_result("()", "char", Err(()), "\"🔥\""),
                            130 => make_any_result("Bytes", "&[u8]", "()", Ok(()), "[1,2,3]"),
                            132 => make_any_result("Bytes", "&[u8]", "()", Err(()), "()"),
                            134 => make_any_result("Slice", "()", "&[char]", Ok(()), "()"),
                            136 => make_slice_result("Prim", "()", "&[char]", Err(()), &["🌊","🦦","🦀"]),
                            _ => panic!("Unexpected i {i}"),
                        }.as_str()
                    );
                    println!("[{i}] {function_name}() -> {return_value}");
                } else if i == 137 {
                    assert!(matches!(return_value, RValue::Unit));
                } else {
                    panic!("{function_name}() {json}");
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn make_result(left: &str, right: &str, is_ok: Result<(), ()>, v: &str) -> String {
    make_any_result("Prim", left, right, is_ok, v)
}

fn make_any_result(ty: &str, left: &str, right: &str, is_ok: Result<(), ()>, v: &str) -> String {
    let is_ok = is_ok.is_ok();
    format!(
        r#"{{"type":"Result","typename":"core::result::Result<{}, {}>","variant":"{}","value":{}}}"#,
        left,
        right,
        if is_ok { "Ok" } else { "Err" },
        if v == "()" {
            r#"{"type":"Unit"}"#.to_owned()
        } else {
            format!(
                r#"{{"type":"{}","typename":"{}","value":{}}}"#,
                ty,
                if is_ok { left } else { right },
                v
            )
        }
    )
}

fn make_slice_result(
    ty: &str,
    left: &str,
    right: &str,
    is_ok: Result<(), ()>,
    vals: &[&str],
) -> String {
    let is_ok = is_ok.is_ok();
    format!(
        r#"{{"type":"Result","typename":"core::result::Result<{}, {}>","variant":"{}","value":{}}}"#,
        left,
        right,
        if is_ok { "Ok" } else { "Err" },
        {
            let val_ty = if is_ok { left } else { right };
            let vals: Vec<_> = vals
                .into_iter()
                .map(|v| {
                    format!(
                        r#"{{"type":"{}","typename":"{}","value":"{}"}}"#,
                        ty,
                        &val_ty[2..val_ty.len() - 1],
                        v
                    )
                })
                .collect();
            format!(
                r#"{{"type":"Array","typename":"slice","data":[{}]}}"#,
                vals.join(",")
            )
        }
    )
}
