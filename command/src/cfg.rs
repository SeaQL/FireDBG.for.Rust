use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Config {
    pub workspace: Workspace,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Workspace {
    pub members: BTreeMap<String, Member>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct Member {
    #[serde(default)]
    pub trace: Trace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Trace {
    #[default]
    None,
    CallOnly,
    Full,
}

impl Trace {
    pub fn to_str(&self) -> &str {
        match self {
            Trace::None => "none",
            Trace::CallOnly => "call-only",
            Trace::Full => "full",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Version {
    pub firedbg_cli: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn firedbg_toml_parsing() -> anyhow::Result<()> {
        let config: Config = toml::from_str(
            r#"
            [workspace.members]
            sea-query = { trace = "full" }
            main-one = { trace = "call-only" } 
            main-two = { trace = "none" } 
            shared = {}
        "#,
        )?;

        assert_eq!(config.workspace.members["sea-query"].trace, Trace::Full);
        assert_eq!(config.workspace.members["main-one"].trace, Trace::CallOnly);
        assert_eq!(config.workspace.members["main-two"].trace, Trace::None);
        assert_eq!(config.workspace.members["shared"].trace, Trace::None);

        Ok(())
    }
}
