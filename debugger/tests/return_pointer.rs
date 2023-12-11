mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "return_pointer";

    let debugger_params = debugger_params_from_file(testcase);

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..16 {
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
                let return_value = return_value.to_string();
                match i {
                    2 => assert_eq!(
                        return_value,
                        r#"alloc::boxed::Box::new(return_pointer::Big { a: 1i32, b: 2i64, c: "box" })"#
                    ),
                    4 => assert_eq!(
                        return_value,
                        r#"alloc::rc::Rc::new(return_pointer::Big { a: 2i32, b: 3i64, c: "rc" })"#
                    ),
                    6 => assert_eq!(
                        return_value,
                        r#"alloc::sync::Arc::new(return_pointer::Big { a: 3i32, b: 4i64, c: "arc" })"#
                    ),
                    8 => assert_eq!(
                        return_value,
                        r#"alloc::boxed::Box::<dyn return_pointer::Mass>::new((?))"#
                    ),
                    10 => assert_eq!(return_value, "alloc::boxed::Box::new(&[0x01, 0x02, 0x03])"),
                    12 => assert_eq!(return_value, "alloc::rc::Rc::new(&[0x01, 0x02, 0x03])"),
                    14 => assert_eq!(return_value, "alloc::sync::Arc::new(&[0x01, 0x02, 0x03])"),
                    15 => assert_eq!(return_value, r#"()"#),
                    _ => panic!("Unexpected i {i}"),
                }
                println!("[{i}] {function_name}() -> {return_value}");
                println!("{json}");
            }
            _ => (),
        }
    }

    Ok(())
}
