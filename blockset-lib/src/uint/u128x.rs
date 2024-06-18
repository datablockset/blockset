use crate::uint::u32x::wadd;

use super::u256x::{self, U256};

/// Converts a 128-bit unsigned integer (`u128`) into a vector of four 32-bit unsigned integers (`[u32; 4]`).
/// This function essentially 'splits' the 128-bit value into a vector of four components, each representing a
/// 32-bit segment, starting from the least significant bits.
#[inline(always)]
pub const fn to_u32x4(v: u128) -> [u32; 4] {
    [
        v as u32,
        (v >> 32) as u32,
        (v >> 64) as u32,
        (v >> 96) as u32,
    ]
}

/// Reconstructs a 128-bit unsigned integer (`u128`) from a vector of four 32-bit unsigned integers (`[u32; 4]`).
/// This operation is the inverse of `to_u32x4`, combining the vector components back into a single 128-bit value.
#[inline(always)]
pub const fn from_u32x4([w0, w1, w2, w3]: [u32; 4]) -> u128 {
    w0 as u128 | ((w1 as u128) << 32) | ((w2 as u128) << 64) | ((w3 as u128) << 96)
}

/// Extracts a single 32-bit component (element) from a 128-bit vector (`u128`) at a specified index.
/// The index `i` determines which 32-bit segment to extract, with `0` being the least significant.
#[inline(always)]
pub const fn get_u32(v: u128, i: usize) -> u32 {
    (v >> (i << 5)) as u32
}

/// Performs element-wise addition of two 128-bit vectors (`u128`), represented as arrays of 32-bit components.
/// Each component of the vectors is added using `add`, which handles overflow by wrapping around.
#[inline(always)]
pub const fn u32x4_wadd(a: u128, b: u128) -> u128 {
    let [a0, a1, a2, a3] = to_u32x4(a);
    let [b0, b1, b2, b3] = to_u32x4(b);
    from_u32x4([wadd(a0, b0), wadd(a1, b1), wadd(a2, b2), wadd(a3, b3)])
}

#[inline(always)]
pub const fn shl(u: u128, i: i32) -> u128 {
    match i {
        -127..=-1 => u >> -i,
        0..=127 => u << i,
        _ => 0,
    }
}

#[inline(always)]
const fn lo_hi(a: u128) -> [u128; 2] {
    [a as u64 as u128, a >> 64]
}

/// Multiplication with overflow.
/// a0 * b0 + (a1 * b0 + a0 * b1) << 64 + (a1 * b1) << 128
pub const fn mul(a: u128, b: u128) -> U256 {
    let [a0, a1] = lo_hi(a);
    let [b0, b1] = lo_hi(b);
    let r0 = [a0 * b0, 0];
    let r1 = {
        let (x, o) = (a1 * b0).overflowing_add(a0 * b1);
        [x << 64, (x >> 64) | ((o as u128) << 64)]
    };
    let r2 = [0, a1 * b1];
    u256x::wadd(u256x::wadd(r0, r1), r2)
}

pub const fn set_bit(a: u128, i: u32) -> u128 {
    a | (1 << i)
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::uint::u128x::{mul, shl};

    #[wasm_bindgen_test]
    #[test]
    fn test_mul() {
        assert_eq!(mul(0, 0), [0, 0]);
        assert_eq!(mul(0, 1), [0, 0]);
        assert_eq!(mul(1, 1), [1, 0]);
        assert_eq!(mul(2, 3), [6, 0]);
        assert_eq!(mul(6, 7), [42, 0]);
        assert_eq!(mul(42, 43), [1_806, 0]);
        assert_eq!(mul(1_806, 1_807), [3_263_442, 0]);
        assert_eq!(mul(3_263_442, 3_263_443), [10_650_056_950_806, 0]);
        assert_eq!(
            mul(10_650_056_950_806, 10_650_056_950_807),
            [113_423_713_055_421_844_361_000_442, 0]
        );
        assert_eq!(
            mul(
                113_423_713_055_421_844_361_000_442,
                113_423_713_055_421_844_361_000_443
            ),
            [
                337_284_947_070_536_250_008_747_159_125_413_113_374,
                37_806_656_864_672
            ]
        );
        assert_eq!(
            mul(u128::MAX, u128::MAX),
            [1, u128::MAX - 1]
        );
    }

    fn check_shl(a: u128, b: i32, expected: u128, f: fn(u128, i32) -> u128) {
        assert_eq!(f(a, b), expected);
    }

    #[wasm_bindgen_test]
    #[test]
    fn shl_test() {
        check_shl(1, -130, 0, shl);
        check_shl(2, -1, 1, shl);
        check_shl(1, 1, 2, shl);
        check_shl(1, 130, 0, shl);
    }
}
