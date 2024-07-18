#[inline(always)]
pub const fn div_rem(a: u64, b: u64) -> (u64, u64) {
    (a / b, a % b)
}

#[inline(always)]
pub const fn swap32(a: u64) -> u64 {
    (a >> 32) | (a << 32)
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::swap32;

    const fn byte_swap(mut a: u64) -> u64 {
        a = swap32(a);
        a = ((a >> 16) & 0x0000FFFF_0000FFFF) | ((a & 0x0000FFFF_0000FFFF) << 16);
        ((a >> 8) & 0x00FF00FF_00FF00FF) | ((a & 0x00FF00FF_00FF00FF) << 8)
    }

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        assert_eq!(byte_swap(0x01234567_89ABCDEF), 0xEFCDAB89_67452301);
    }
}