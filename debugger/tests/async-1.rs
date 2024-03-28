mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "async-1";
    let (producer, consumer) = setup(testcase).await?;

    let mut debugger_params = debugger_params_testbench(testcase);

    println!("{:#?}", debugger_params.breakpoints);

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..100 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        match &event {
            Event::Breakpoint { .. } => (),
            Event::FunctionCall { function_name, .. } => {}
            Event::FunctionReturn { function_name, .. } => {}
        }
    }

    Ok(())
}

// function end }
// 1. without {{closure}}: enter future
// 2. with {{closure}}: exit future
//
// function {
// 1. everytime future is re-entered

// #0 FunctionCall { breakpoint_id: 9, thread_id: 12475492, frame_id: 1, stack_pointer: 6123578480, function_name: "main::main", arguments: [] }
// #1 FunctionCall { breakpoint_id: 5, thread_id: 12475492, frame_id: 2, stack_pointer: 6123573392, function_name: "main::uid", arguments: [] }
// #2 FunctionReturn { breakpoint_id: 29, thread_id: 12475492, frame_id: 2, function_name: "main::uid", return_value: Prim(u64(5581)) }
// #3 FunctionCall { breakpoint_id: 7, thread_id: 12475492, frame_id: 3, stack_pointer: 6123573408, function_name: "main::sleep2", arguments: [("ctx", Prim(u64(5581)))] }
// #4 FunctionReturn { breakpoint_id: 30, thread_id: 12475492, frame_id: 3, function_name: "main::sleep2", return_value: Opaque }
// #5 FunctionCall { breakpoint_id: 8, thread_id: 12475492, frame_id: 4, stack_pointer: 6123573408, function_name: "main::sleep3", arguments: [("ctx", Prim(u64(5581)))] }
// #6 FunctionReturn { breakpoint_id: 31, thread_id: 12475492, frame_id: 4, function_name: "main::sleep3", return_value: Opaque }
// #7 FunctionCall { breakpoint_id: 7, thread_id: 12475492, frame_id: 5, stack_pointer: 6123571040, function_name: "main::sleep2::{{closure}}", arguments: [] }
// #8 FunctionCall { breakpoint_id: 5, thread_id: 12475492, frame_id: 6, stack_pointer: 6123571008, function_name: "main::uid", arguments: [] }
// #9 FunctionReturn { breakpoint_id: 29, thread_id: 12475492, frame_id: 6, function_name: "main::uid", return_value: Prim(u64(6411)) }
// #10 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 7, stack_pointer: 6123571024, function_name: "main::sleeper", arguments: [("ctx", Prim(u64(6411))), ("i", Prim(u64(1)))] }
// #11 FunctionReturn { breakpoint_id: 35, thread_id: 12475492, frame_id: 7, function_name: "main::sleeper", return_value: Opaque }
// #12 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 8, stack_pointer: 6123570160, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #13 FunctionCall { breakpoint_id: 5, thread_id: 12475492, frame_id: 9, stack_pointer: 6123570128, function_name: "main::uid", arguments: [] }
// #14 FunctionReturn { breakpoint_id: 29, thread_id: 12475492, frame_id: 9, function_name: "main::uid", return_value: Prim(u64(3181)) }
// #15 FunctionReturn { breakpoint_id: 36, thread_id: 12475492, frame_id: 8, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #16 FunctionReturn { breakpoint_id: 32, thread_id: 12475492, frame_id: 5, function_name: "main::sleep2::{{closure}}", return_value: Opaque }
// #17 FunctionCall { breakpoint_id: 8, thread_id: 12475492, frame_id: 10, stack_pointer: 6123571584, function_name: "main::sleep3::{{closure}}", arguments: [] }
// #18 FunctionCall { breakpoint_id: 5, thread_id: 12475492, frame_id: 11, stack_pointer: 6123571552, function_name: "main::uid", arguments: [] }
// #19 FunctionReturn { breakpoint_id: 29, thread_id: 12475492, frame_id: 11, function_name: "main::uid", return_value: Prim(u64(3079)) }
// #20 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 12, stack_pointer: 6123571568, function_name: "main::sleeper", arguments: [("ctx", Prim(u64(3079))), ("i", Prim(u64(3)))] }
// #21 FunctionReturn { breakpoint_id: 35, thread_id: 12475492, frame_id: 12, function_name: "main::sleeper", return_value: Opaque }
// #22 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 13, stack_pointer: 6123570704, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #23 FunctionCall { breakpoint_id: 5, thread_id: 12475492, frame_id: 14, stack_pointer: 6123570672, function_name: "main::uid", arguments: [] }
// #24 FunctionReturn { breakpoint_id: 29, thread_id: 12475492, frame_id: 14, function_name: "main::uid", return_value: Prim(u64(9970)) }
// #25 FunctionReturn { breakpoint_id: 36, thread_id: 12475492, frame_id: 13, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #26 FunctionReturn { breakpoint_id: 38, thread_id: 12475492, frame_id: 10, function_name: "main::sleep3::{{closure}}", return_value: Opaque }
// #27 FunctionCall { breakpoint_id: 8, thread_id: 12475492, frame_id: 15, stack_pointer: 6123571584, function_name: "main::sleep3::{{closure}}", arguments: [] }
// #28 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 16, stack_pointer: 6123570704, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #29 FunctionReturn { breakpoint_id: 37, thread_id: 12475492, frame_id: 16, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #30 FunctionReturn { breakpoint_id: 39, thread_id: 12475492, frame_id: 15, function_name: "main::sleep3::{{closure}}", return_value: Opaque }
// #31 FunctionCall { breakpoint_id: 7, thread_id: 12475492, frame_id: 17, stack_pointer: 6123571040, function_name: "main::sleep2::{{closure}}", arguments: [] }
// #32 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 18, stack_pointer: 6123570160, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #33 FunctionReturn { breakpoint_id: 37, thread_id: 12475492, frame_id: 18, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #34 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 19, stack_pointer: 6123571024, function_name: "main::sleeper", arguments: [("ctx", Prim(u64(6411))), ("i", Prim(u64(1)))] }
// #35 FunctionReturn { breakpoint_id: 35, thread_id: 12475492, frame_id: 19, function_name: "main::sleeper", return_value: Opaque }
// #36 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 20, stack_pointer: 6123570160, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #37 FunctionCall { breakpoint_id: 5, thread_id: 12475492, frame_id: 21, stack_pointer: 6123570128, function_name: "main::uid", arguments: [] }
// #38 FunctionReturn { breakpoint_id: 29, thread_id: 12475492, frame_id: 21, function_name: "main::uid", return_value: Prim(u64(1054)) }
// #39 FunctionReturn { breakpoint_id: 36, thread_id: 12475492, frame_id: 20, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #40 FunctionReturn { breakpoint_id: 33, thread_id: 12475492, frame_id: 17, function_name: "main::sleep2::{{closure}}", return_value: Opaque }
// #41 FunctionCall { breakpoint_id: 7, thread_id: 12475492, frame_id: 22, stack_pointer: 6123571040, function_name: "main::sleep2::{{closure}}", arguments: [] }
// #42 FunctionCall { breakpoint_id: 6, thread_id: 12475492, frame_id: 23, stack_pointer: 6123570160, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #43 FunctionReturn { breakpoint_id: 37, thread_id: 12475492, frame_id: 23, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #44 FunctionReturn { breakpoint_id: 34, thread_id: 12475492, frame_id: 22, function_name: "main::sleep2::{{closure}}", return_value: Opaque }
// #45 FunctionReturn { breakpoint_id: 10, thread_id: 12475492, frame_id: 1, function_name: "main::main", return_value: Unit }
