pub const fn to_ascii(x: char) -> Option<u8> {
    if x <= '\u{7F}' {
        Some(x as u8)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::to_ascii;

    #[inline(never)]
    fn x(x: char, y: Option<u8>, f: fn(char) -> Option<u8>) {
        assert_eq!(f(x), y);
        if let Some(y) = y {
            assert_eq!(f(char::from_u32(x as u32 / 2).unwrap()), Some(y / 2));
        } else {
            assert_eq!(f(char::from_u32(x as u32 / 2).unwrap()), None);
        }
    }

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        x('a', Some(97), to_ascii);
        x('🦀', None, to_ascii);
    }
}
