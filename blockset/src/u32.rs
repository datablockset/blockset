#[inline(always)]
pub const fn add(a: u32, b: u32) -> u32 {
    a.overflowing_add(b).0
}

#[inline(always)]
pub const fn add2(a: u32, b: u32, c: u32) -> u32 {
    add(add(a, b), c)
}

#[inline(always)]
pub const fn add3(a: u32, b: u32, c: u32, d: u32) -> u32 {
    add(add(a, b), add(c, d))
}

#[inline(always)]
pub const fn add4(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    add2(add2(a, b, c), d, e)
}

#[inline(always)]
pub const fn to_u8x4(a: u32) -> [u8; 4] {
    [a as u8, (a >> 8) as u8, (a >> 16) as u8, (a >> 24) as u8]
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        assert_eq!(super::add(1, 2), 3);
        assert_eq!(super::add2(1, 2, 3), 6);
        assert_eq!(super::add3(1, 2, 3, 4), 10);
        assert_eq!(super::add4(1, 2, 3, 4, 5), 15);
        assert_eq!(super::to_u8x4(0x12345678), [0x78, 0x56, 0x34, 0x12]);
    }
}
