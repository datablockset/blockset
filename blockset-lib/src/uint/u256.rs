use crate::uint::u128::{to_u32x4, u32x4_add};

pub type U256 = [u128; 2];

#[inline(always)]
pub const fn u32x8_add(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [u32x4_add(a0, b0), u32x4_add(a1, b1)]
}

pub const fn shl(&[lo, hi]: &U256, i: usize) -> U256 {
    if i < 128 {
        if i == 0 {
            [lo, hi]
        } else {
            [lo << i, (hi << i) | ((lo >> (128 - i)) & ((1 << i) - 1))]
        }
    } else {
        // If i >= 256, a standard `<<` function should panic in debug mode and return the original
        // value in release mode. The `shl` function returns 0 in both modes.
        [0, if i >= 256 { 0 } else { lo << (i - 128) }]
    }
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

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{shl, U256};

    #[wasm_bindgen_test]
    #[test]
    fn shl_test() {
        const X: U256 = [
            0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201,
            0x201F_1E1D_1C1B_1A19_1817_1615_1413_1211,
        ];
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
        assert_eq!(shl(&X, 128), [0, 0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201]);
        assert_eq!(shl(&X, 129), [0, 0x201E_1C1A_1816_1412_100E_0C0A_0806_0402]);
        assert_eq!(shl(&X, 136), [0, 0x0F_0E0D_0C0B_0A09_0807_0605_0403_020100]);
        assert_eq!(shl(&X, 248), [0, 0x0100_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(shl(&X, 255), [0, 0x8000_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(shl(&X, 256), [0; 2]);
    }
}
