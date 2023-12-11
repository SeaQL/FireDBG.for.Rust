use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref CONFIG: HashMap<String, ConfigValue> = {
        dotenvy::from_filename(".env.local").ok();
        dotenvy::from_filename(".env").ok();
        let mut map = HashMap::new();
        if let Ok(string) = std::env::var("FIREDBG_RUST_CONFIG") {
            for pair in string.split(";") {
                let mut pair = pair.split("=");
                if let Some(key) = pair.next() {
                    let key = key.to_owned();
                    if let Some(value) = pair.next() {
                        if matches!(value, "true" | "false") {
                            map.insert(key, ConfigValue::bool(value == "true"));
                        } else if let Ok(v) = value.parse::<u64>() {
                            map.insert(key, ConfigValue::u64(v));
                        } else {
                            map.insert(key, ConfigValue::String(value.to_owned()));
                        }
                    } else {
                        map.insert(key, ConfigValue::bool(true));
                    }
                }
            }
        }
        map
    };
    /// Maximum number of items in array, string and other containers.
    pub static ref MAX_ARRAY_SIZE: usize = config_usize("MAX_ARRAY_SIZE").unwrap_or(1024);
    /// Recursive limit; i.e. this limits the depth of a binary tree.
    pub static ref RECURSIVE_DEREF_LIMIT: usize = config_usize("RECURSIVE_DEREF_LIMIT").unwrap_or(12);
    /// If set, don't sort hash maps by hash key.
    pub static ref KEEP_HASH_ORDER: bool = config_bool("KEEP_HASH_ORDER");
    /// If set, don't trace heap allocations.
    pub static ref DONT_TRACE_ALLOCATION: bool = config_bool("DONT_TRACE_ALLOCATION");
}

#[doc(hidden)]
/// This initializes the global debugger config.
pub fn load_config() -> usize {
    CONFIG.len()
}

fn config_usize(key: &str) -> Option<usize> {
    match CONFIG.get(key).cloned() {
        Some(ConfigValue::u64(v)) => Some(v as usize),
        _ => None,
    }
}

/// Unset is false
fn config_bool(key: &str) -> bool {
    match CONFIG.get(key).cloned() {
        Some(ConfigValue::bool(v)) => v,
        _ => false,
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
enum ConfigValue {
    String(String),
    bool(bool),
    u64(u64),
}
