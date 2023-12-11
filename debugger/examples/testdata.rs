//! A small utility to generate test data
use anyhow::Result;
use firedbg_rust_debugger::{ActiveFrame, BpId, EventStream, EVENT_STREAM};
use lldb::SBFunctionId;
use sea_streamer::{Producer, SeaConnectOptions, SeaStreamer, StreamKey, Streamer};

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = "file://./testdata.ss";
    let mut options = SeaConnectOptions::default();
    options.set_file_connect_options(|options| {
        options.set_end_with_eos(true);
    });
    let streamer = SeaStreamer::connect(file_path.parse()?, options).await?;
    let stream_key = StreamKey::new(EVENT_STREAM)?;
    let producer = streamer
        .create_producer(stream_key, Default::default())
        .await?;
    let mut rand = random(12345678);
    let mut state = State::new();

    for i in 0..10000 {
        let is_fn_call = i < 4 || rand.next().unwrap() % 10 < 6; // bias towards function call
        let mut event = if is_fn_call {
            let fid = state.i();
            EventStream::function_call(
                BpId(1),
                1,
                &ActiveFrame {
                    frame_id: fid,
                    stack_pointer: 0xffff,
                    program_counter: 0xffff,
                    function_name: format!("fn_{fid}"),
                    function_id: SBFunctionId(0), // don't care
                },
            )
        } else {
            let fid = state.o();
            if fid.is_none() {
                println!("oopps! seems like we ended early");
                break;
            }
            let fid = fid.unwrap();
            EventStream::function_return(
                BpId(1),
                1,
                &ActiveFrame {
                    frame_id: fid,
                    stack_pointer: 0xffff,
                    program_counter: 0xffff,
                    function_name: format!("fn_{fid}"),
                    function_id: SBFunctionId(0), // don't care
                },
            )
        };
        if is_fn_call {
            event.write_unit_v("value");
        } else {
            if state.i % 6 == 0 {
                event.write_opaque_v("return_value");
            } else {
                event.write_unit_v("return_value");
            }
        }
        producer.send(event)?;
        if state.i == 2500 {
            break;
        }
    }

    // unwind the stack
    while let Some(fid) = state.o() {
        let mut event = EventStream::function_return(
            BpId(1),
            1,
            &ActiveFrame {
                frame_id: fid,
                stack_pointer: 0xffff,
                program_counter: 0xffff,
                function_name: format!("fn_{fid}"),
                function_id: SBFunctionId(0), // don't care
            },
        );
        if fid % 6 == 0 {
            event.write_opaque_v("return_value");
        } else {
            event.write_unit_v("return_value");
        }
        producer.send(event)?;
    }

    producer.end().await?;

    Ok(())
}

// Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
//
// https://github.com/rust-lang/rust/blob/1.55.0/library/core/src/slice/sort.rs#L559-L573
fn random(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

/// Simulate the debugger state
struct State {
    i: u64,
    s: Vec<u64>,
}

impl State {
    fn new() -> Self {
        Self {
            i: 0,
            s: Vec::new(),
        }
    }

    /// in
    fn i(&mut self) -> u64 {
        self.i += 1;
        self.s.push(self.i);
        self.i
    }

    /// out
    fn o(&mut self) -> Option<u64> {
        self.s.pop()
    }
}
