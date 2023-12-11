mod util;
use util::*;

use anyhow::Result;
use firedbg_rust_debugger::{
    Breakpoint, BreakpointType, Bytes, Debugger, EventStream, LineColumn, VariableCapture,
};
use pretty_assertions::assert_eq;
use sea_streamer::{Buffer, Consumer, Message, Producer};

#[tokio::test]
async fn main() -> Result<()> {
    let testcase = "quicksort";
    let (producer, consumer) = setup(testcase).await?;

    let debugger_params = debugger_params_from_file(testcase);

    // println!("{:#?}", debugger_params.breakpoints);

    assert_eq!(
        debugger_params.breakpoints[1],
        Breakpoint {
            id: 1,
            file_id: 1,
            loc: LineColumn {
                line: 3,
                column: Some(12),
            },
            loc_end: Some(LineColumn {
                line: 4,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "main".into(),
            },
            capture: VariableCapture::Arguments,
        }
    );
    assert_eq!(
        debugger_params.breakpoints[2],
        Breakpoint {
            id: 2,
            file_id: 1,
            loc: LineColumn {
                line: 11,
                column: Some(40),
            },
            loc_end: Some(LineColumn {
                line: 12,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "try_return".into(),
            },
            capture: VariableCapture::Arguments,
        },
    );
    assert_eq!(
        debugger_params.breakpoints[3],
        Breakpoint {
            id: 3,
            file_id: 1,
            loc: LineColumn {
                line: 15,
                column: Some(47),
            },
            loc_end: Some(LineColumn {
                line: 16,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "run".into(),
            },
            capture: VariableCapture::Arguments,
        },
    );
    assert_eq!(
        debugger_params.breakpoints[4],
        Breakpoint {
            id: 4,
            file_id: 1,
            loc: LineColumn {
                line: 20,
                column: Some(79),
            },
            loc_end: Some(LineColumn {
                line: 21,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "quick_sort".into(),
            },
            capture: VariableCapture::Arguments,
        },
    );
    assert_eq!(
        debugger_params.breakpoints[5],
        Breakpoint {
            id: 5,
            file_id: 1,
            loc: LineColumn {
                line: 30,
                column: Some(79),
            },
            loc_end: Some(LineColumn {
                line: 31,
                column: Some(5)
            }),
            breakpoint_type: BreakpointType::FunctionCall {
                fn_name: "partition".into(),
            },
            capture: VariableCapture::Arguments,
        },
    );

    Debugger::run(debugger_params, producer.clone());

    producer.end().await?;

    for i in 0..42 {
        let payload = consumer.next().await?.message().into_bytes();
        let event = EventStream::read_from(Bytes::from(payload));
        println!("#{i} {:?}", event);

        // TODO: Assert the result
    }

    Ok(())
}
