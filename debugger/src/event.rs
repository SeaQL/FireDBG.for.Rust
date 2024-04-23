use crate::{Bytes, Event, Reader};

#[derive(Debug)]
/// Event Stream
pub struct EventStream {}

#[cfg(feature = "debugger")]
#[derive(Debug)]
/// The current stack frame
pub struct ActiveFrame {
    pub frame_id: u64,
    /// Aka SP
    pub stack_pointer: u64,
    /// Aka PC
    pub program_counter: u64,
    pub function_name: String,
    pub function_id: lldb::SBFunctionId,
}

impl EventStream {
    pub fn read_from(source: Bytes) -> Event {
        let mut reader = Reader::new();
        assert!(source.len() > 0);
        let event = match source.get(0) {
            b'B' => {
                let mut i = 1;
                let reason = match source.get(i) {
                    b'B' => crate::Reason::Breakpoint,
                    b'P' => crate::Reason::Panic,
                    b'F' => {
                        i += 1;
                        match source.get(i) {
                            b'{' => crate::Reason::FutureEnter,
                            b'}' => crate::Reason::FutureExit,
                            other => panic!("Unknown reason, got {other:?}"),
                        }
                    }
                    other => panic!("Unknown reason, got {other:?}"),
                };
                i += 1;
                reader.set_source(source, i);
                let breakpoint_id = reader.read_int().unwrap() as u32;
                let thread_id = reader.read_int().unwrap();
                let frame_id = reader.read_int().unwrap();
                let locals = reader.read_values();
                Event::Breakpoint {
                    breakpoint_id,
                    thread_id,
                    frame_id,
                    reason,
                    locals,
                }
            }
            b'F' => {
                reader.set_source(source, 1);
                let breakpoint_id = reader.read_int().unwrap() as u32;
                let thread_id = reader.read_int().unwrap();
                let frame_id = reader.read_int().unwrap();
                let stack_pointer = reader.read_int().unwrap();
                let function_name = reader.read_string().unwrap();
                let arguments = reader.read_values();
                Event::FunctionCall {
                    breakpoint_id,
                    thread_id,
                    frame_id,
                    stack_pointer,
                    function_name,
                    arguments,
                }
            }
            b'R' => {
                reader.set_source(source, 1);
                let breakpoint_id = reader.read_int().unwrap() as u32;
                let thread_id = reader.read_int().unwrap();
                let frame_id = reader.read_int().unwrap();
                let function_name = reader.read_string().unwrap();
                let mut values = reader.read_values();
                let (name, return_value) = values.remove(0);
                assert_eq!(name, "return_value");
                Event::FunctionReturn {
                    breakpoint_id,
                    thread_id,
                    frame_id,
                    function_name,
                    return_value,
                }
            }
            o => panic!("Unknown Event {o:?}"),
        };
        event
    }
}

#[cfg(feature = "debugger")]
impl EventStream {
    pub fn breakpoint(
        bp_id: crate::BpId,
        thread_id: u64,
        frame_id: u64,
        reason: crate::Reason,
    ) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.push_byte(b'B');
        bytes.push_byte(match reason {
            crate::Reason::Breakpoint => b'B',
            crate::Reason::Panic => b'P',
            crate::Reason::FutureEnter => b'F',
            crate::Reason::FutureExit => b'F',
        });
        match reason {
            crate::Reason::FutureEnter => bytes.push_byte(b'{'),
            crate::Reason::FutureExit => bytes.push_byte(b'}'),
            _ => (),
        }
        bytes.integer(bp_id.0);
        bytes.integer(thread_id);
        bytes.integer(frame_id);
        bytes
    }

    pub fn function_call(bp_id: crate::BpId, thread_id: u64, active_frame: &ActiveFrame) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.push_byte(b'F');
        bytes.integer(bp_id.0);
        bytes.integer(thread_id);
        bytes.integer(active_frame.frame_id);
        bytes.integer(active_frame.stack_pointer);
        bytes.identifier(&active_frame.function_name);
        bytes
    }

    pub fn function_return(
        bp_id: crate::BpId,
        thread_id: u64,
        active_frame: &ActiveFrame,
    ) -> Bytes {
        let mut bytes = Bytes::new();
        bytes.push_byte(b'R');
        bytes.integer(bp_id.0);
        bytes.integer(thread_id);
        bytes.integer(active_frame.frame_id);
        bytes.identifier(&active_frame.function_name);
        bytes
    }
}
