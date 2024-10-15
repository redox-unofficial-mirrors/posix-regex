pub fn is_alnum(c: u8) -> bool {
    is_alpha(c) || is_digit(c)
}
pub fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic()
}
pub fn is_blank(c: u8) -> bool {
    c == b' ' || c == b'\t'
}
pub fn is_cntrl(c: u8) -> bool {
    c.is_ascii_control()
}
pub fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}
pub fn is_graph(c: u8) -> bool {
    c.is_ascii_graphic()
}
pub fn is_lower(c: u8) -> bool {
    c.is_ascii_lowercase()
}
pub fn is_print(c: u8) -> bool {
    (0x20..=0x7e).contains(&c)
}
pub fn is_punct(c: u8) -> bool {
    is_graph(c) && !is_alnum(c)
}
pub fn is_space(c: u8) -> bool {
    c == b' ' || (0x9..=0xD).contains(&c)
}
pub fn is_upper(c: u8) -> bool {
    c.is_ascii_uppercase()
}
pub fn is_xdigit(c: u8) -> bool {
    c.is_ascii_hexdigit()
}

pub fn is_word_boundary(c: u8) -> bool {
    !is_alnum(c) && c != b'_'
}
