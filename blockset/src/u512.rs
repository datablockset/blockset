use crate::u256::U256;

pub type U512 = [U256; 2];

pub const fn new(a: u128, b: u128, c: u128, d: u128) -> U512 {
    [[a, b], [c, d]]
}

pub const fn get_u128(a: &U512, i: usize) -> u128 {
    a[(i >> 1) & 1][i & 1]
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::new;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let a = new(1, 2, 3, 4);
        assert_eq!(a[0][0], 1);
        assert_eq!(a[0][1], 2);
        assert_eq!(a[1][0], 3);
        assert_eq!(a[1][1], 4);
    }
}
