pub const fn to_ascii(x: char) -> Option<u8> {
    if x <= '\u{7F}' {
        Some(x as u8)
    } else {
        None
    }
}
