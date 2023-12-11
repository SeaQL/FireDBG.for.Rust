mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, EventStream, LineColumn, VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[ignore]
#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "quicksort_perf";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    // for i in 0..1000 {
    //     let payload = consumer.next().await?.message().into_bytes();
    //     let event = EventStream::read_from(Bytes::from(payload));
    //     println!("#{i} {:?}", event);

    //     // TODO: Assert the result
    // }

    Ok(())
}
