/// A very crappy implementation for test case validation.
///
/// ```ignore
/// assert!(wildcard_match("abc..", "abcdefg"));
/// assert!(wildcard_match("..efg", "abcdefg"));
/// assert!(wildcard_match("hello..world..!", "hello 100 worlds!"));
/// ```
pub fn wildcard_match(template: &str, against: &str) -> bool {
    let parts: Vec<&str> = template.split("..").collect();
    if parts.len() == 1 {
        return template == against;
    }
    if !against.starts_with(parts[0]) {
        return false;
    }
    let mut index = parts[0].len().max(1);
    for i in 1..parts.len() - 1 {
        let part = parts[i];
        if let Some(at) = against[index..].find(part) {
            if i > 0 && at == 0 {
                // .. means one or more char
                return false;
            }
            index += at + part.len();
        } else {
            return false;
        }
    }
    if let Some(part) = parts.last() {
        if part.is_empty() {
            // template ends with ..
            return index < against.len();
        } else {
            return against[index..].ends_with(part);
        }
    }
    index == against.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wildcard_0() {
        assert!(wildcard_match("abc", "abc"));
        assert!(!wildcard_match("abc", "abcd"));
        assert!(!wildcard_match("abcd", "abc"));
    }

    #[test]
    fn test_wildcard_1() {
        assert!(wildcard_match("abc..", "abcdefg"));
        assert!(!wildcard_match("abc..", "abc"));
        assert!(wildcard_match("..efg", "abcdefg"));
        assert!(!wildcard_match("..efg", "efg"));
        assert!(wildcard_match("ab..c", "ab____c"));
        assert!(wildcard_match("ab..c", "ab!c"));
        assert!(!wildcard_match("ab..c", "a___bc"));
        assert!(!wildcard_match("ab..c", "ab____cd"));
        assert!(wildcard_match("(..)", "({})"));
        assert!(wildcard_match("(..)", "(())"));
    }

    #[test]
    fn test_wildcard_2() {
        assert!(wildcard_match("hello..world..!", "hello world !"));
        assert!(!wildcard_match("hello..world..!", "helloworld!"));
        assert!(wildcard_match("hello..world..!", "hello 100 worlds!"));
        assert!(!wildcard_match("hello..world..!", "hello world"));
        assert!(!wildcard_match("hello..world..!", "hello worlds"));
        assert!(wildcard_match("hello..world..", "hello world~~~"));
        assert!(wildcard_match("..hello..world", "happy hello world"));
        assert!(wildcard_match(
            "..hello..world..",
            "happy hello world forever"
        ));
        assert!(!wildcard_match("..hello..world..", "hello world"));
    }
}
