mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "os_string";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);
    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..4 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match event {
            Event::Breakpoint { .. } => unreachable!(),
            Event::FunctionCall { arguments, .. } => {
                if i == 1 {
                    let string = arguments[0].1.to_string();
                    assert_eq!(
                        string,
                        r#"&std::path::PathBuf { inner: std::ffi::os_str::OsString::from_encoded_bytes_unchecked(String::from("/home/hello").into_bytes()) }"#
                    );
                }
            }
            Event::FunctionReturn { .. } => {}
        }
    }

    Ok(())
}
