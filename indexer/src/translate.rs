use crate::entity::{
    allocation::ActiveModel as Allocation,
    breakpoint::{self, Model as Breakpoint},
    debugger_info::ActiveModel as DebuggerInfo,
    event::{ActiveModel as Event, EventType},
    file::ActiveModel as File,
    type_info::Model as TypeInfo,
};
use firedbg_rust_debugger::{
    Allocation as SrcAllocation, Breakpoint as SrcBreakPoint, DebuggerInfo as SrcDebuggerInfo,
    Event as SrcEvent, InfoMessage as SrcInfoMessage, ProgExitInfo as SrcProgExitInfo, RValue,
    Reason, SourceFile,
};
use sea_orm::{prelude::DateTimeUtc, IntoActiveModel, NotSet, Set};
use sea_streamer::Timestamp;
use serde::Serialize;
use std::fmt::Write;

pub fn debugger_info(info: SrcInfoMessage) -> DebuggerInfo {
    match info {
        SrcInfoMessage::Debugger(SrcDebuggerInfo {
            debugger,
            version,
            workspace_root,
            package_name,
            target,
            arguments,
        }) => DebuggerInfo {
            id: NotSet,
            debugger: Set(debugger.to_string()),
            version: Set(version),
            workspace_root: Set(workspace_root),
            package_name: Set(package_name),
            target: Set(target),
            arguments: Set(json_stringify(&arguments)),
            exit_code: Set(None),
        },
        SrcInfoMessage::Exit(SrcProgExitInfo { exit_code }) => DebuggerInfo {
            id: Set(1),
            exit_code: Set(Some(exit_code)),
            ..Default::default()
        },
    }
}

pub fn source_file(f: SourceFile) -> File {
    let SourceFile {
        id,
        path,
        crate_name,
        modified,
    } = f;
    let modified: DateTimeUtc = modified.into();
    File {
        id: Set(id),
        path: Set(path),
        crate_name: Set(crate_name),
        modified: Set(modified.format("%Y-%m-%d %H:%M:%S").to_string()),
    }
}

pub fn breakpoint(bp: SrcBreakPoint) -> breakpoint::ActiveModel {
    let SrcBreakPoint {
        id,
        file_id,
        loc,
        loc_end: _,
        breakpoint_type: bp_type,
        capture,
    } = bp;

    Breakpoint {
        id,
        file_id,
        loc_line: loc.line,
        loc_column: loc.column,
        breakpoint_type: json_stringify(&bp_type),
        capture: json_stringify(&capture),
        hit_count: 0,
    }
    .into_active_model()
}

pub fn event(timestamp: Timestamp, event: SrcEvent) -> Event {
    match event {
        SrcEvent::Breakpoint {
            breakpoint_id,
            thread_id,
            frame_id,
            reason,
            locals,
        } => {
            let thread_id = thread_id as i64;
            let frame_id = frame_id as i64;
            let data = json_stringify(&locals);
            let mut pretty = String::new();
            let mut is_error = false;
            write!(pretty, "(").unwrap();
            if !locals.is_empty() {
                write!(pretty, "\n").unwrap();
            }
            for (name, value) in locals {
                write!(pretty, "{name}: {value:#},\n").unwrap();
                is_error |= value_is_error(&value);
            }
            write!(pretty, ")").unwrap();
            let event_type = match reason {
                Reason::Breakpoint => EventType::Breakpoint,
                Reason::Panic => {
                    is_error = true;
                    EventType::Panic
                }
            };

            Event {
                id: NotSet,
                breakpoint_id: Set(breakpoint_id),
                thread_id: Set(thread_id),
                frame_id: Set(frame_id),
                parent_frame_id: NotSet,
                stack_pointer: Set(None),
                function_name: Set(None),
                event_type: Set(event_type),
                timestamp: Set(timestamp),
                data: Set(data),
                pretty: Set(pretty),
                is_error: Set(is_error),
            }
        }
        SrcEvent::FunctionCall {
            breakpoint_id,
            thread_id,
            frame_id,
            stack_pointer,
            function_name,
            arguments,
        } => {
            let thread_id = thread_id as i64;
            let frame_id = frame_id as i64;
            let stack_pointer = stack_pointer as i64;
            let data = json_stringify(&arguments);
            let mut pretty = String::new();
            let mut is_error = false;
            write!(pretty, "(").unwrap();
            if !arguments.is_empty() {
                write!(pretty, "\n").unwrap();
            }
            for (name, value) in arguments {
                write!(pretty, "{name}: {value:#},\n").unwrap();
                is_error |= value_is_error(&value);
            }
            write!(pretty, ")").unwrap();

            Event {
                id: NotSet,
                breakpoint_id: Set(breakpoint_id),
                thread_id: Set(thread_id),
                frame_id: Set(frame_id),
                parent_frame_id: NotSet,
                stack_pointer: Set(Some(stack_pointer)),
                function_name: Set(Some(function_name)),
                event_type: Set(EventType::FunctionCall),
                timestamp: Set(timestamp),
                data: Set(data),
                pretty: Set(pretty),
                is_error: Set(is_error),
            }
        }
        SrcEvent::FunctionReturn {
            breakpoint_id,
            thread_id,
            frame_id,
            function_name,
            return_value,
        } => {
            let thread_id = thread_id as i64;
            let frame_id = frame_id as i64;
            let data = json_stringify(&return_value);
            let pretty = format!("{return_value:#}");
            let is_error = value_is_error(&return_value);

            Event {
                id: NotSet,
                breakpoint_id: Set(breakpoint_id),
                thread_id: Set(thread_id),
                frame_id: Set(frame_id),
                parent_frame_id: NotSet,
                stack_pointer: Set(None),
                function_name: Set(Some(function_name)),
                event_type: Set(EventType::FunctionReturn),
                timestamp: Set(timestamp),
                data: Set(data),
                pretty: Set(pretty),
                is_error: Set(is_error),
            }
        }
    }
}

pub fn type_info<F: FnMut(TypeInfo)>(event: &SrcEvent, mut push: F) {
    match event {
        SrcEvent::Breakpoint { locals, .. } => {
            locals.iter().for_each(|(_, v)| {
                if let Some(ty) = type_info_of(v) {
                    push(ty);
                }
            });
        }
        SrcEvent::FunctionCall { arguments, .. } => {
            arguments.iter().for_each(|(_, v)| {
                if let Some(ty) = type_info_of(v) {
                    push(ty);
                }
            });
        }
        SrcEvent::FunctionReturn { return_value, .. } => {
            if let Some(ty) = type_info_of(return_value) {
                push(ty);
            }
        }
    }
}

fn type_info_of(value: &RValue) -> Option<TypeInfo> {
    let type_name = value.typename();
    if type_name.starts_with("alloc::") {
        return None;
    }
    Some(TypeInfo {
        type_name,
        attributes: match value {
            RValue::Struct { fields, .. } => {
                let mut s = "[".to_string();
                for (i, name) in fields.keys().enumerate() {
                    write!(s, "{}\"{}\"", if i == 0 { "" } else { "," }, name).unwrap();
                }
                write!(s, "]").unwrap();
                Some(s)
            }
            RValue::Union { typeinfo, .. } => Some(json_stringify(&typeinfo.variants)),
            _ => None,
        },
    })
}

pub fn allocation(bp: SrcAllocation) -> Allocation {
    let SrcAllocation {
        action,
        address,
        type_name,
    } = bp;

    Allocation {
        id: NotSet,
        action: Set(action.to_string()),
        address: Set(address as i64),
        type_name: Set(type_name),
    }
}

fn value_is_error(value: &RValue) -> bool {
    if value.is_result() {
        value.result_variant().is_err()
    } else {
        false
    }
}

fn json_stringify<T>(v: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(v).expect("Fail to serialize into JSON string")
}
