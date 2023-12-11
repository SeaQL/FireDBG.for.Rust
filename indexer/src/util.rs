use firedbg_rust_debugger::{DebuggerInfo, InfoMessage};
use sea_streamer::{Buffer, Message, SharedMessage};

pub fn deser<T: serde::de::DeserializeOwned>(m: &SharedMessage) -> T {
    try_deser(m).expect("Deserialization failed")
}

fn try_deser<T: serde::de::DeserializeOwned>(m: &SharedMessage) -> Result<T, String> {
    m.message().deserialize_json().map_err(|e| {
        format!(
            "Failed to deserialize message `{}`: {}",
            m.message()
                .as_str()
                .expect("Failed to convert message to &str"),
            e
        )
    })
}

pub fn deser_info(m: &SharedMessage) -> InfoMessage {
    if let Ok(info) = try_deser::<InfoMessage>(m) {
        info
    } else {
        let info = deser::<DebuggerInfo>(m);
        InfoMessage::Debugger(info)
    }
}
