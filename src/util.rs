pub fn is_ident(c: u8) -> bool {
    (b'0'..=b'9').contains(&c)
        || (b'A'..=b'Z').contains(&c)
        || (b'a'..=b'z').contains(&c)
        || c == b'-'
        || c == b'_'
}
