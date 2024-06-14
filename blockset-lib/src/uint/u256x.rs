use crate::uint::u128x::{to_u32x4, u32x4_wadd};

use super::{
    u128x,
    u512x::{self, U512},
};

pub type U256 = [u128; 2];

#[inline(always)]
pub const fn u32x8_wadd(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [u32x4_wadd(a0, b0), u32x4_wadd(a1, b1)]
}

#[inline(always)]
pub const fn shl(&[lo, hi]: &U256, i: i32) -> U256 {
    [
        u128x::shl(lo, i),
        u128x::shl(hi, i) | u128x::shl(lo, i - 128),
    ]
}

#[inline(always)]
pub const fn bitor(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [a0 | b0, a1 | b1]
}

#[inline(always)]
pub const fn eq(&[a0, a1]: &U256, &[b0, b1]: &U256) -> bool {
    a0 == b0 && a1 == b1
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

pub const fn oadd([a0, a1]: U256, [b0, b1]: U256) -> (U256, bool) {
    let (r0, c) = a0.overflowing_add(b0);
    let (a1c, c0) = a1.overflowing_add(c as u128);
    let (r1, c1) = a1c.overflowing_add(b1);
    assert!(!(c0 & c1));
    ([r0, r1], c0 | c1)
}

#[inline(always)]
pub const fn wadd(a: U256, b: U256) -> U256 {
    oadd(a, b).0
}

pub const fn osub([a0, a1]: U256, [b0, b1]: U256) -> (U256, bool) {
    let (r0, c) = a0.overflowing_sub(b0);
    let (a1c, c0) = a1.overflowing_sub(c as u128);
    let (r1, c1) = a1c.overflowing_sub(b1);
    assert!(!(c0 & c1));
    ([r0, r1], c0 | c1)
}

#[inline(always)]
pub const fn wsub(a: U256, b: U256) -> U256 {
    osub(a, b).0
}

#[inline(always)]
pub const fn from_u128(a: u128) -> U256 {
    [a, 0]
}

#[inline(always)]
pub const fn from_bool(a: bool) -> U256 {
    from_u128(a as u128)
}

pub const ZERO: U256 = from_u128(0);

pub const fn leading_zeros([a0, a1]: U256) -> u32 {
    match a1.leading_zeros() {
        128 => 128 + a0.leading_zeros(),
        x => x,
    }
}

pub const fn mul([a0, a1]: U256, [b0, b1]: U256) -> U512 {
    let r0 = [u128x::mul(a0, b0), ZERO];
    let r1 = {
        let [x0, x1] = wadd(u128x::mul(a1, b0), u128x::mul(a0, b1));
        [[0, x0], [x1, 0]]
    };
    let r2 = [ZERO, u128x::mul(a1, b1)];
    u512x::wadd(u512x::wadd(r0, r1), r2)
}

pub fn div(a: U256, b: U256) -> (U256, U256) {
    if less(&a, &b) {
        return (ZERO, a);
    }
    (ZERO, a)
}

pub const fn set_bit([a0, a1]: U256, i: u32) -> U256 {
    if i < 128 {
        [u128x::set_bit(a0, i), a1]
    } else {
        [a0, u128x::set_bit(a1, i - 128)]
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::uint::u256x::{from_u128, leading_zeros, mul, osub, wadd, ZERO};

    use super::{shl, U256};

    #[test]
    #[wasm_bindgen_test]
    fn test_leading_zeros() {
        assert_eq!(leading_zeros(ZERO), 256);
        assert_eq!(leading_zeros(from_u128(1)), 255);
        assert_eq!(leading_zeros([0, 1]), 127);
        assert_eq!(leading_zeros([0, 1 << 127]), 0);
    }

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
        assert_eq!(
            mul(
                [
                    5_371_062_831_933_040_625_063_535_520_489_472_000,
                    301_007_701_468_498_206_271_604_815_495_196_034_759
                ],
                [1_134_551_116_646_504_048_636_759_994_223_388, 1]
            ),
            [
                [
                    60_254_530_117_125_738_360_543_440_126_070_226_944,
                    59_522_230_753_612_122_465_001_774_279_148_829_737
                ],
                [301_008_705_072_143_350_860_077_419_384_325_574_160, 0]
            ]
        );
        assert_eq!(
            mul(
                [
                    60_254_530_117_125_738_360_543_440_126_070_226_944,
                    59_522_230_753_612_122_465_001_774_279_148_829_737
                ],
                [301_008_705_072_143_350_860_077_419_384_325_574_160, 1]
            ),
            [
                [
                    9_558_019_270_435_171_971_601_763_919_821_012_992,
                    95_671_844_947_069_765_262_691_166_418_826_886_560
                ],
                [112_174_708_060_239_291_868_245_242_763_017_670_789, 0]
            ]
        );
        assert_eq!(
            mul(
                [
                    9_558_019_270_435_171_971_601_763_919_821_012_992,
                    95_671_844_947_069_765_262_691_166_418_826_886_560
                ],
                [
                    112_174_708_060_239_291_868_245_242_763_017_670_789,
                    301_008_705_072_143_350_860_077_419_384_325_574_160
                ]
            ),
            [
                [
                    124_520_065_329_156_727_566_853_617_294_340_259_840,
                    85_178_092_293_149_724_732_751_559_228_462_861_597
                ],
                [
                    143_515_348_539_978_426_307_971_476_180_421_425_837,
                    84_629_886_702_508_214_136_893_118_870_499_585_684
                ]
            ]
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_add() {
        assert_eq!(wadd([0, 0], [0, 0]), [0, 0]);
        assert_eq!(wadd([0, 1], [0, 2]), [0, 3]);
        assert_eq!(wadd([1, 2], [3, 4]), [4, 6]);
        assert_eq!(wadd([u128::MAX, 3], [4, 5]), [3, 9]);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_osub() {
        assert_eq!(osub([0, 0], [0, 0]), ([0, 0], false));
        assert_eq!(osub([0, 1], [0, 2]), ([0, u128::MAX], true));
        assert_eq!(osub([0, 2], [0, 1]), ([0, 1], false));
        assert_eq!(osub([1, 2], [3, 4]), ([u128::MAX - 1, u128::MAX - 2], true));
        assert_eq!(osub([3, 4], [1, 2]), ([2, 2], false));
        assert_eq!(
            osub([u128::MAX, 3], [4, 5]),
            ([u128::MAX - 4, u128::MAX - 1], true)
        );
        assert_eq!(osub([4, 5], [u128::MAX, 3]), ([5, 1], false));
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
