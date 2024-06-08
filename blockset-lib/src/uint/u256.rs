use crate::uint::u128::{shl as shl128, to_u32x4, u32x4_add};

use super::u512::{self, U512};

pub type U256 = [u128; 2];

#[inline(always)]
pub const fn u32x8_add(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [u32x4_add(a0, b0), u32x4_add(a1, b1)]
}

#[inline(always)]
pub const fn shl(&[lo, hi]: &U256, i: usize) -> U256 {
    [
        shl128(lo, i as i32),
        shl128(hi, i as i32) | shl128(lo, i as i32 - 128),
    ]
}

#[inline(always)]
pub const fn bitor(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [a0 | b0, a1 | b1]
}

// Don't use `<` for `U256` because it's not LE comparison.
#[inline(always)]
pub const fn less(&[a0, a1]: &U256, &[b0, b1]: &U256) -> bool {
    if a1 == b1 {
        a0 < b0
    } else {
        a1 < b1
    }
}

#[inline(always)]
pub const fn great(a: &U256, b: &U256) -> bool {
    less(b, a)
}

pub const fn to_u224(&[a0, a1]: &U256) -> Option<[u32; 7]> {
    let [a10, a11, a12, a13] = to_u32x4(a1);
    if a13 != 0xFFFF_FFFF {
        return None;
    }
    let [a00, a01, a02, a03] = to_u32x4(a0);
    Some([a00, a01, a02, a03, a10, a11, a12])
}

pub const fn overflowing_add([a0, a1]: U256, [b0, b1]: U256) -> (U256, bool) {
    let (r0, c) = a0.overflowing_add(b0);
    let (a1c, c0) = a1.overflowing_add(c as u128);
    let (r1, c1) = a1c.overflowing_add(b1);
    assert!(!(c0 & c1));
    ([r0, r1], c0 | c1)
}

pub const fn add(a: U256, b: U256) -> U256 {
    overflowing_add(a, b).0
}

pub const ZERO: U256 = [0, 0];

pub const fn mul([a0, a1]: U256, [b0, b1]: U256) -> U512 {
    let r0 = [super::u128::mul(a0, b0), ZERO];
    let r1 = {
        let [x0, x1] = add(super::u128::mul(a1, b0), super::u128::mul(a0, b1));
        [[0, x0], [x1, 0]]
    };
    let r2 = [ZERO, super::u128::mul(a1, b1)];
    u512::add(u512::add(r0, r1), r2)
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::uint::u256::{add, mul};

    use super::{shl, U256};

    #[test]
    #[wasm_bindgen_test]
    fn test_mul() {
        assert_eq!(mul([1, 2], [3, 4]), [[3, 10], [8, 0]]);
        assert_eq!(mul([3, 10], [8, 1]), [[24, 83], [10, 0]]);
        assert_eq!(mul([24, 83], [10, 1]), [[240, 854], [83, 0]]);
        assert_eq!(mul([240, 854], [83, 1]), [[19_920, 71_122], [854, 0]]);
        assert_eq!(
            mul([19_920, 71_122], [854, 1]),
            [[17_011_680, 60_758_108], [71_122, 0]]
        );
        assert_eq!(
            mul([17_011_680, 60_758_108], [71_122, 1]),
            [[1_209_904_704_960, 4_321_255_168_856], [60_758_108, 0]]
        );
        assert_eq!(
            mul([1_209_904_704_960, 4_321_255_168_856], [60_758_108, 1]),
            [
                [73_511_520_733_667_815_680, 262_551_289_454_815_789_408,],
                [4_321_255_168_856, 0]
            ]
        );
        assert_eq!(
            mul(
                [73_511_520_733_667_815_680, 262_551_289_454_815_789_408,],
                [4_321_255_168_856, 1]
            ),
            [
                [
                    317_662_038_940_827_061_850_491_084_462_080,
                    1_134_551_116_646_504_047_761_375_644_092_928
                ],
                [262_551_289_454_815_789_408, 0]
            ]
        );
        assert_eq!(
            mul(
                [
                    317_662_038_940_827_061_850_491_084_462_080,
                    1_134_551_116_646_504_047_761_375_644_092_928
                ],
                [262_551_289_454_815_789_408, 1]
            ),
            [
                [
                    5_371_062_831_933_040_625_063_535_520_489_472_000,
                    301_007_701_468_498_206_271_604_815_495_196_034_759
                ],
                [1_134_551_116_646_504_048_636_759_994_223_388, 0]
            ]
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_add() {
        assert_eq!(add([0, 0], [0, 0]), [0, 0]);
        assert_eq!(add([0, 1], [0, 2]), [0, 3]);
        assert_eq!(add([1, 2], [3, 4]), [4, 6]);
        assert_eq!(add([u128::MAX, 3], [4, 5]), [3, 9]);
    }

    const X: U256 = [
        0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201,
        0x201F_1E1D_1C1B_1A19_1817_1615_1413_1211,
    ];

    #[wasm_bindgen_test]
    #[test]
    fn shl_test() {
        assert_eq!(shl(&X, 0), X);
        assert_eq!(
            shl(&X, 1),
            // 123456789ABCDEF
            // 000000011111111
            // 2468ACE02468ACE
            [
                0x201E_1C1A_1816_1412_100E_0C0A_0806_0402,
                0x403E_3C3A_3836_3432_302E_2C2A_2826_2422,
            ]
        );
        assert_eq!(
            shl(&X, 4),
            [
                0x00F_0E0D_0C0B_0A09_0807_0605_0403_0201_0,
                0x01F_1E1D_1C1B_1A19_1817_1615_1413_1211_1,
            ]
        );
        assert_eq!(
            shl(&X, 124),
            [
                0x1000_0000_0000_0000_0000_0000_0000_0000,
                0x1100F_0E0D_0C0B_0A09_0807_0605_0403_020,
            ]
        );
        assert_eq!(
            shl(&X, 127),
            [
                0x8000_0000_0000_0000_0000_0000_0000_0000,
                0x8807_8706_8605_8504_8403_8302_8201_8100,
            ]
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn shl_test2() {
        assert_eq!(shl(&X, 128), [0, 0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201]);
        assert_eq!(shl(&X, 129), [0, 0x201E_1C1A_1816_1412_100E_0C0A_0806_0402]);
        assert_eq!(shl(&X, 136), [0, 0x0F_0E0D_0C0B_0A09_0807_0605_0403_020100]);
        assert_eq!(shl(&X, 248), [0, 0x0100_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(shl(&X, 255), [0, 0x8000_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(shl(&X, 256), [0; 2]);
    }
}
