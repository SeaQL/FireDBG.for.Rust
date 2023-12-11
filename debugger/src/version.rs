/// Get the `rustc` version *in the system*.
pub fn rustc_version() -> String {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let output = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    parse_rustc_version(stdout)
}

fn parse_rustc_version(line: &str) -> String {
    let full_version = line.split_whitespace().nth(1).expect("full version");
    let mut version_parts: Vec<_> = full_version.split('.').rev().collect();
    assert!(version_parts.len() > 2);
    let major = version_parts.pop().expect("major version");
    let minor = version_parts.pop().expect("minor version");
    format!("{major}.{minor}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rustc_version() {
        assert_eq!(
            parse_rustc_version("rustc 1.73.0 (cc66ad468 2023-10-03)"),
            "1.73"
        );
        assert_eq!(
            parse_rustc_version("rustc 1.72.1 (xxxxxxxxx 2023-09-19)"),
            "1.72"
        );
        assert_eq!(
            parse_rustc_version("rustc 1.72.0 (xxxxxxxxx 2023-08-24)"),
            "1.72"
        );
        assert_eq!(
            parse_rustc_version("rustc 1.71.1 (xxxxxxxxx 2023-08-03)"),
            "1.71"
        );
        assert_eq!(
            parse_rustc_version("rustc 1.71.0 (xxxxxxxxx 2023-07-13)"),
            "1.71"
        );
        assert_eq!(
            parse_rustc_version("rustc 1.70.0 (xxxxxxxxx 2023-06-01)"),
            "1.70"
        );
    }
}
