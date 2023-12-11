mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, DebuggerParams, EventStream, LineColumn,
    SourceFile, VariableCapture,
};
use sea_streamer::{Buffer, Consumer, Message, Producer};
use std::time::SystemTime;

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "fn_return";

    // this is the only way llvm would generate multiple return instructions
    rustc_optimize(&format!("testcases/{testcase}"));

    let debugger_params = DebuggerParams {
        binary: format!("testcases/{testcase}.o"),
        files: vec![
            Default::default(),
            SourceFile {
                id: 1,
                path: format!("testcases/{testcase}.rs"),
                crate_name: testcase.into(),
                modified: SystemTime::UNIX_EPOCH,
            },
        ],
        breakpoints: vec![
            Default::default(),
            Breakpoint {
                id: 1,
                file_id: 1,
                loc: LineColumn {
                    line: 27,
                    column: None,
                },
                loc_end: None,
                breakpoint_type: BreakpointType::FunctionCall {
                    fn_name: "main".into(),
                },
                capture: VariableCapture::Arguments,
            },
            Breakpoint {
                id: 2,
                file_id: 1,
                loc: LineColumn {
                    line: 2,
                    column: None,
                },
                loc_end: None,
                breakpoint_type: BreakpointType::FunctionCall {
                    fn_name: "multi_return".into(),
                },
                capture: VariableCapture::Arguments,
            },
        ],
        arguments: vec![],
    };

    let (producer, consumer) = setup(testcase).await?;

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    let expected = vec![
        Expected::FnCall {
            name: "main".into(),
            args: vec![],
        },
        Expected::FnCall {
            name: "multi_return".into(),
            args: vec!["1i32".to_owned()],
        },
        Expected::FnRet {
            name: "multi_return".into(),
            value: "11i32".into(),
        },
        Expected::FnCall {
            name: "multi_return".into(),
            args: vec!["2i32".to_owned()],
        },
        Expected::FnRet {
            name: "multi_return".into(),
            value: "22i32".into(),
        },
        Expected::FnCall {
            name: "multi_return".into(),
            args: vec!["3i32".to_owned()],
        },
        Expected::FnRet {
            name: "multi_return".into(),
            value: "33i32".into(),
        },
        Expected::FnCall {
            name: "multi_return".into(),
            args: vec!["4i32".to_owned()],
        },
        Expected::FnRet {
            name: "multi_return".into(),
            value: "0i32".into(),
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
