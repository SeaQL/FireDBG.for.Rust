/// Parse an assembly into (inst, operand)
pub(super) fn parse_asm(long: &str) -> (&str, &str) {
    if let Some(pv) = long.find(char::is_whitespace) {
        let inst = &long[..pv];
        let operand = long[pv..].trim();
        (inst, operand)
    } else {
        (long, "")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_asm() {
        assert_eq!(
            parse_asm("sub    sp, sp, #0x180"),
            ("sub", "sp, sp, #0x180")
        );
        assert_eq!(
            parse_asm("str    w1, [sp, #0x8]"),
            ("str", "w1, [sp, #0x8]")
        );
        assert_eq!(
            parse_asm("stur    w0, [x29, #-0xa0]"),
            ("stur", "w0, [x29, #-0xa0]")
        );
        assert_eq!(parse_asm("ret"), ("ret", ""));
    }
}
