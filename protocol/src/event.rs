//! Data structures for Debugger Event
use crate::value::RValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Debugger Event
pub enum Event {
    Breakpoint {
        breakpoint_id: u32,
        thread_id: u64,
        frame_id: u64,
        reason: Reason,
        locals: Vec<(String, RValue)>,
    },
    FunctionCall {
        breakpoint_id: u32,
        thread_id: u64,
        frame_id: u64,
        stack_pointer: u64,
        function_name: String,
        arguments: Vec<(String, RValue)>,
    },
    FunctionReturn {
        breakpoint_id: u32,
        thread_id: u64,
        frame_id: u64,
        function_name: String,
        return_value: RValue,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
/// Reason a breakpoint is hit.
pub enum Reason {
    #[default]
    Breakpoint,
    Panic,
    FutureEnter,
    FutureExit,
}

impl Event {
    /// Print arguments in the form of "name: value, .."
    ///
    /// # Panics
    ///
    /// Panic if self is not FunctionCall
    pub fn format_arguments(&self) -> String {
        use std::fmt::Write;

        match self {
            Event::FunctionCall { arguments, .. } => {
                let mut string = String::new();
                for (i, (name, value)) in arguments.iter().enumerate() {
                    write!(string, "{}{name}: {value}", if i > 0 { ", " } else { "" }).unwrap();
                }
                string
            }
            _ => panic!("Not FunctionCall"),
        }
    }

    /// Redact the memory address of all values. Intended for testing.
    pub fn redacted(&mut self) {
        match self {
            Event::Breakpoint {
                breakpoint_id,
                thread_id,
                locals,
                ..
            } => {
                *breakpoint_id = u32::MAX;
                *thread_id = u64::MAX;
                for (_, ref mut local) in locals.iter_mut() {
                    local.redact_addr();
                }
            }
            Event::FunctionCall {
                breakpoint_id,
                thread_id,
                stack_pointer,
                arguments,
                ..
            } => {
                *breakpoint_id = u32::MAX;
                *thread_id = u64::MAX;
                *stack_pointer = u64::MAX;
                for (_, ref mut argument) in arguments.iter_mut() {
                    argument.redact_addr();
                }
            }
            Event::FunctionReturn {
                breakpoint_id,
                thread_id,
                return_value,
                ..
            } => {
                *breakpoint_id = u32::MAX;
                *thread_id = u64::MAX;
                return_value.redact_addr();
            }
        }
    }
}
