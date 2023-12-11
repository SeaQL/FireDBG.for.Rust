mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "fn_prologue";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..12 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        // println!("#{i} {:?}", event);

        match &event {
            Event::FunctionCall {
                function_name,
                arguments,
                ..
            } => {
                assert_eq!(
                    function_name,
                    match i {
                        0 => "fn_prologue::main",
                        1 => "fn_prologue::no_prolog",
                        3 => "fn_prologue::prolog",
                        4 => "fn_prologue::dice",
                        6 => "fn_prologue::dice",
                        8 => "fn_prologue::dice",
                        _ => panic!("Unexpected i {i}"),
                    }
                );
                match i {
                    1 => {
                        assert_eq!(arguments.len(), 1);
                        assert_eq!(arguments[0].1.to_string(), "12345678i32");
                    }
                    3 => {
                        assert_eq!(arguments.len(), 3);
                        assert_eq!(arguments[0].1.to_string(), "1234i32");
                        assert_eq!(arguments[1].1.to_string(), "5678i32");
                        assert_eq!(arguments[2].1.to_string(), "12345678i32");
                    }
                    _ => (),
                }
                println!("[{i}] {function_name}({})", event.format_arguments());
            }
            Event::FunctionReturn {
                function_name,
                return_value,
                ..
            } => {
                let return_value = return_value.to_string();
                assert_eq!(
                    return_value,
                    match i {
                        2 => "()",
                        5 => "core::result::Result::<i32, ()>::Ok(1234i32)",
                        7 => "core::result::Result::<i32, ()>::Ok(5678i32)",
                        9 => "core::result::Result::<i32, ()>::Ok(12345678i32)",
                        10 => "core::result::Result::<(), ()>::Ok(())",
                        11 => "()",
                        i => panic!("Unexpected i {i}"),
                    }
                );
                println!("[{i}] {function_name}(..) -> {return_value}");
            }
            _ => (),
        }
    }

    Ok(())
}
