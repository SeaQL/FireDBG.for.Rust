mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_option";

    let debugger_params = debugger_params_from_file(testcase);

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..102 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);

        match event {
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => {
                let return_value = return_value.to_string();
                assert_eq!(
                    return_value,
                    match i {
                        2 => "core::option::Option::<()>::None",
                        4 => "core::option::Option::<()>::Some(())",
                        6 => "core::option::Option::<bool>::None",
                        8 => "core::option::Option::<bool>::Some(true)",
                        10 => "core::option::Option::<i8>::None",
                        12 => "core::option::Option::<i8>::Some(-22i8)",
                        14 => "core::option::Option::<u8>::None",
                        16 => "core::option::Option::<u8>::Some(250u8)",
                        18 => "core::option::Option::<i16>::None",
                        20 => "core::option::Option::<i16>::Some(-22222i16)",
                        22 => "core::option::Option::<u16>::None",
                        24 => "core::option::Option::<u16>::Some(65432u16)",
                        26 => "core::option::Option::<i32>::None",
                        28 => "core::option::Option::<i32>::Some(-222222i32)",
                        30 => "core::option::Option::<u32>::None",
                        32 => "core::option::Option::<u32>::Some(432432u32)",
                        34 => "core::option::Option::<i64>::None",
                        36 => "core::option::Option::<i64>::Some(-22222222222i64)",
                        38 => "core::option::Option::<u64>::None",
                        40 => "core::option::Option::<u64>::Some(23232232232u64)",
                        42 => "core::option::Option::<isize>::None",
                        44 => "core::option::Option::<isize>::Some(-22222222222isize)",
                        46 => "core::option::Option::<usize>::None",
                        48 => "core::option::Option::<usize>::Some(23232232232usize)",
                        50 => "core::option::Option::<i128>::None",
                        52 => "core::option::Option::<i128>::Some(-22222222222222222222i128)",
                        54 => "core::option::Option::<u128>::None",
                        56 => "core::option::Option::<u128>::Some(33333333333333333333u128)",
                        58 => "core::option::Option::<f32>::None",
                        60 => "core::option::Option::<f32>::Some(111.111f32)",
                        62 => "core::option::Option::<f64>::None",
                        64 => "core::option::Option::<f64>::Some(222.222f64)",
                        66 => r#"core::option::Option::<&str>::None"#,
                        68 => r#"core::option::Option::<&str>::Some("hello")"#,
                        70 => "core::option::Option::<char>::None",
                        72 => "core::option::Option::<char>::Some('@')",
                        74 => "core::option::Option::<&[u8]>::None",
                        76 => "core::option::Option::<&[u8]>::Some(&[0x01, 0x02, 0x03])",
                        78 => "core::option::Option::<&[char]>::None",
                        80 => r#"core::option::Option::<&[char]>::Some(&['🌊', '🦦', '🦀'])"#,
                        82 => r#"core::option::Option::<return_option::Car>::None"#,
                        84 =>
                            r#"core::option::Option::<return_option::Car>::Some(return_option::Car { brand: "Ford", engine: return_option::Engine { config: return_option::EngineConfig::Inline { i: 4i32 }, pistons: vec![return_option::Piston(1u8), return_option::Piston(2u8), return_option::Piston(3u8), return_option::Piston(4u8)] }, gearbox: return_option::Gearbox::Manual })"#,
                        86 =>
                            r#"core::option::Option::<&return_option::Car>::Some(&return_option::Car { brand: "Mazda", engine: return_option::Engine { config: return_option::EngineConfig::Vshape(3i16, 3i16), pistons: vec![] }, gearbox: return_option::Gearbox::Automatic })"#,
                        88 => r#"core::option::Option::<return_option::Small>::None"#,
                        90 => r#"core::option::Option::<return_option::Small>::Some(12345678i32)"#,
                        92 =>
                            r#"core::option::Option::<alloc::boxed::Box<dyn return_option::Auto>>::None"#,
                        94 =>
                            r#"core::option::Option::<alloc::boxed::Box<dyn return_option::Auto>>::Some(alloc::boxed::Box::<dyn return_option::Auto>::new((?)))"#,
                        96 => "core::option::Option::<return_option::Gearbox>::None",
                        98 => "core::option::Option::<return_option::Gearbox>::Some(return_option::Gearbox::Automatic)",
                        100 => "core::option::Option::<return_option::Gearbox>::Some(return_option::Gearbox::Manual)",
                        101 => "()",
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
