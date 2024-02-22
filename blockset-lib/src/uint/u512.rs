use crate::uint::u256::U256;

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

    #[inline(never)]
    fn create(a: u128, b: u128, c: u128, d: u128) {
        for i in 0..4 {
            let x = new(a * i, b, c / (i + 1), d);
            assert_eq!(x[0][0], a * i);
            assert_eq!(x[0][1], b);
            assert_eq!(x[1][0], c / (i + 1));
            assert_eq!(x[1][1], d);
        }
    }

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        create(1, 2, 3, 4);
    }
}
