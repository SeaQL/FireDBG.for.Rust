mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{Bytes, Debugger, Event, EventStream};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[ignore]
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

// #0 FunctionCall { breakpoint_id: 12, thread_id: <>, frame_id: 1, stack_pointer: 6168093696, function_name: "main::main", arguments: [] }
// #1 FunctionCall { breakpoint_id: 5, thread_id: <>, frame_id: 2, stack_pointer: 6168088608, function_name: "main::uid", arguments: [] }
// #2 FunctionReturn { breakpoint_id: 33, thread_id: <>, frame_id: 2, function_name: "main::uid", return_value: Prim(u64(9938)) }
// #3 FunctionCall { breakpoint_id: 8, thread_id: <>, frame_id: 3, stack_pointer: 6168088624, function_name: "main::sleep2", arguments: [("ctx", Prim(u64(9938)))] }
// #4 Breakpoint { breakpoint_id: 9, thread_id: <>, frame_id: 3, reason: FutureEnter, locals: [("fn", String { typename: StrLit, value: "main::sleep2" })] }
// #5 FunctionReturn { breakpoint_id: 34, thread_id: <>, frame_id: 3, function_name: "main::sleep2", return_value: Opaque }
// #6 FunctionCall { breakpoint_id: 10, thread_id: <>, frame_id: 4, stack_pointer: 6168088624, function_name: "main::sleep3", arguments: [("ctx", Prim(u64(9938)))] }
// #7 Breakpoint { breakpoint_id: 11, thread_id: <>, frame_id: 4, reason: FutureEnter, locals: [("fn", String { typename: StrLit, value: "main::sleep3" })] }
// #8 FunctionReturn { breakpoint_id: 35, thread_id: <>, frame_id: 4, function_name: "main::sleep3", return_value: Opaque }
// #9 FunctionCall { breakpoint_id: 8, thread_id: <>, frame_id: 5, stack_pointer: 6168086256, function_name: "main::sleep2::{{closure}}", arguments: [] }
// #10 FunctionCall { breakpoint_id: 5, thread_id: <>, frame_id: 6, stack_pointer: 6168086224, function_name: "main::uid", arguments: [] }
// #11 FunctionReturn { breakpoint_id: 33, thread_id: <>, frame_id: 6, function_name: "main::uid", return_value: Prim(u64(9473)) }
// #12 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 7, stack_pointer: 6168086240, function_name: "main::sleeper", arguments: [("ctx", Prim(u64(9473))), ("i", Prim(u64(1)))] }
// #13 Breakpoint { breakpoint_id: 7, thread_id: <>, frame_id: 7, reason: FutureEnter, locals: [("fn", String { typename: StrLit, value: "main::sleeper" })] }
// #14 FunctionReturn { breakpoint_id: 39, thread_id: <>, frame_id: 7, function_name: "main::sleeper", return_value: Opaque }
// #15 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 8, stack_pointer: 6168085376, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #16 FunctionCall { breakpoint_id: 5, thread_id: <>, frame_id: 9, stack_pointer: 6168085344, function_name: "main::uid", arguments: [] }
// #17 FunctionReturn { breakpoint_id: 33, thread_id: <>, frame_id: 9, function_name: "main::uid", return_value: Prim(u64(9232)) }
// #18 FunctionReturn { breakpoint_id: 40, thread_id: <>, frame_id: 8, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #19 FunctionReturn { breakpoint_id: 36, thread_id: <>, frame_id: 5, function_name: "main::sleep2::{{closure}}", return_value: Opaque }
// #20 FunctionCall { breakpoint_id: 10, thread_id: <>, frame_id: 10, stack_pointer: 6168086800, function_name: "main::sleep3::{{closure}}", arguments: [] }
// #21 FunctionCall { breakpoint_id: 5, thread_id: <>, frame_id: 11, stack_pointer: 6168086768, function_name: "main::uid", arguments: [] }
// #22 FunctionReturn { breakpoint_id: 33, thread_id: <>, frame_id: 11, function_name: "main::uid", return_value: Prim(u64(7131)) }
// #23 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 12, stack_pointer: 6168086784, function_name: "main::sleeper", arguments: [("ctx", Prim(u64(7131))), ("i", Prim(u64(3)))] }
// #24 Breakpoint { breakpoint_id: 7, thread_id: <>, frame_id: 12, reason: FutureEnter, locals: [("fn", String { typename: StrLit, value: "main::sleeper" })] }
// #25 FunctionReturn { breakpoint_id: 39, thread_id: <>, frame_id: 12, function_name: "main::sleeper", return_value: Opaque }
// #26 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 13, stack_pointer: 6168085920, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #27 FunctionCall { breakpoint_id: 5, thread_id: <>, frame_id: 14, stack_pointer: 6168085888, function_name: "main::uid", arguments: [] }
// #28 FunctionReturn { breakpoint_id: 33, thread_id: <>, frame_id: 14, function_name: "main::uid", return_value: Prim(u64(1699)) }
// #29 FunctionReturn { breakpoint_id: 40, thread_id: <>, frame_id: 13, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #30 FunctionReturn { breakpoint_id: 42, thread_id: <>, frame_id: 10, function_name: "main::sleep3::{{closure}}", return_value: Opaque }
// #31 FunctionCall { breakpoint_id: 10, thread_id: <>, frame_id: 15, stack_pointer: 6168086800, function_name: "main::sleep3::{{closure}}", arguments: [] }
// #32 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 16, stack_pointer: 6168085920, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #33 FunctionReturn { breakpoint_id: 40, thread_id: <>, frame_id: 16, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #34 FunctionReturn { breakpoint_id: 42, thread_id: <>, frame_id: 15, function_name: "main::sleep3::{{closure}}", return_value: Opaque }
// #35 FunctionCall { breakpoint_id: 8, thread_id: <>, frame_id: 17, stack_pointer: 6168086256, function_name: "main::sleep2::{{closure}}", arguments: [] }
// #36 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 18, stack_pointer: 6168085376, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #37 Breakpoint { breakpoint_id: 7, thread_id: <>, frame_id: 18, reason: FutureExit, locals: [("fn", String { typename: StrLit, value: "main::sleeper" })] }
// #38 FunctionReturn { breakpoint_id: 41, thread_id: <>, frame_id: 18, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #39 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 19, stack_pointer: 6168086240, function_name: "main::sleeper", arguments: [("ctx", Prim(u64(9473))), ("i", Prim(u64(1)))] }
// #40 Breakpoint { breakpoint_id: 7, thread_id: <>, frame_id: 19, reason: FutureEnter, locals: [("fn", String { typename: StrLit, value: "main::sleeper" })] }
// #41 FunctionReturn { breakpoint_id: 39, thread_id: <>, frame_id: 19, function_name: "main::sleeper", return_value: Opaque }
// #42 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 20, stack_pointer: 6168085376, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #43 FunctionCall { breakpoint_id: 5, thread_id: <>, frame_id: 21, stack_pointer: 6168085344, function_name: "main::uid", arguments: [] }
// #44 FunctionReturn { breakpoint_id: 33, thread_id: <>, frame_id: 21, function_name: "main::uid", return_value: Prim(u64(7518)) }
// #45 FunctionReturn { breakpoint_id: 40, thread_id: <>, frame_id: 20, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #46 FunctionReturn { breakpoint_id: 37, thread_id: <>, frame_id: 17, function_name: "main::sleep2::{{closure}}", return_value: Opaque }
// #47 FunctionCall { breakpoint_id: 8, thread_id: <>, frame_id: 22, stack_pointer: 6168086256, function_name: "main::sleep2::{{closure}}", arguments: [] }
// #48 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 23, stack_pointer: 6168085376, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #49 Breakpoint { breakpoint_id: 7, thread_id: <>, frame_id: 23, reason: FutureExit, locals: [("fn", String { typename: StrLit, value: "main::sleeper" })] }
// #50 FunctionReturn { breakpoint_id: 41, thread_id: <>, frame_id: 23, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #51 Breakpoint { breakpoint_id: 9, thread_id: <>, frame_id: 22, reason: FutureExit, locals: [("fn", String { typename: StrLit, value: "main::sleep2" })] }
// #52 FunctionReturn { breakpoint_id: 38, thread_id: <>, frame_id: 22, function_name: "main::sleep2::{{closure}}", return_value: Opaque }
// #53 FunctionCall { breakpoint_id: 10, thread_id: <>, frame_id: 24, stack_pointer: 6168086800, function_name: "main::sleep3::{{closure}}", arguments: [] }
// #54 FunctionCall { breakpoint_id: 6, thread_id: <>, frame_id: 25, stack_pointer: 6168085920, function_name: "main::sleeper::{{closure}}", arguments: [] }
// #55 Breakpoint { breakpoint_id: 7, thread_id: <>, frame_id: 25, reason: FutureExit, locals: [("fn", String { typename: StrLit, value: "main::sleeper" })] }
// #56 FunctionReturn { breakpoint_id: 41, thread_id: <>, frame_id: 25, function_name: "main::sleeper::{{closure}}", return_value: Opaque }
// #57 Breakpoint { breakpoint_id: 11, thread_id: <>, frame_id: 24, reason: FutureExit, locals: [("fn", String { typename: StrLit, value: "main::sleep3" })] }
// #58 FunctionReturn { breakpoint_id: 43, thread_id: <>, frame_id: 24, function_name: "main::sleep3::{{closure}}", return_value: Opaque }
// #59 FunctionReturn { breakpoint_id: 14, thread_id: <>, frame_id: 1, function_name: "main::main", return_value: Unit }
