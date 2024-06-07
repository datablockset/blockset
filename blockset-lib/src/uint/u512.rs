use crate::uint::u256::U256;

use super::u256;

pub type U512 = [U256; 2];

pub const fn new(a: u128, b: u128, c: u128, d: u128) -> U512 {
    [[a, b], [c, d]]
}

pub const fn get_u128(a: &U512, i: usize) -> u128 {
    a[(i >> 1) & 1][i & 1]
}

pub const fn add([a0, a1]: U512, [b0, b1]: U512) -> U512 {
    let (r0, c) = u256::overflowing_add(a0, b0);
    [r0, u256::add(u256::add(a1, b1), [c as u128, 0])]
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{new, U512};

    //#[inline(never)]
    fn create2(
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        i: u128,
        f: fn(u128, u128, u128, u128) -> U512,
    ) -> U512 {
        f(a * i, b + i, c / (i + 1), d - 1)
    }

    //#[inline(never)]
    fn create(a: u128, b: u128, c: u128, d: u128) {
        for i in 0..10 {
            let x = create2(a, b, c, d, i, new);
            assert_eq!(x[0][0], a * i);
            assert_eq!(x[0][1], b + i);
            assert_eq!(x[1][0], c / (i + 1));
            assert_eq!(x[1][1], d - 1);
            let xa = new(a, b + i, c / (i + 1), d - 1);
            assert_eq!(xa[0][0], a);
            assert_eq!(xa[0][1], b + i);
            assert_eq!(xa[1][0], c / (i + 1));
            assert_eq!(xa[1][1], d - 1);
        }
    }

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        create(1, 2, 3, 4);
    }
}
