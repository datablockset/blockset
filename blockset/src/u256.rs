use crate::{
    u128::{to_u32x4, u32x4_add},
    u512::U512,
};

pub type U256 = [u128; 2];

#[inline(always)]
pub const fn u32x8_add(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [u32x4_add(a0, b0), u32x4_add(a1, b1)]
}

pub const fn to_u32x8([a, b]: &U256) -> [u32; 8] {
    let [a0, a1, a2, a3] = to_u32x4(*a);
    let [b0, b1, b2, b3] = to_u32x4(*b);
    [a0, a1, a2, a3, b0, b1, b2, b3]
}

pub const fn to_u512(a: &U256) -> U512 {
    [*a, [0, 0]]
}

pub const fn shl(&[a, b]: &U256, i: usize) -> U256 {
    if i < 128 {
        [a << i, (b << i) | ((a >> (128 - i)) & ((1 << i) - 1))]
    } else if i < 256 {
        [0, a << (i - 128)]
    } else {
        [a, b]
    }
}

#[cfg(test)]
mod test {
    use crate::u256::{shl, U256};

    #[test]
    fn test() {
        const X: U256 = [
            0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201,
            0x201F_1E1D_1C1B_1A19_1817_1615_1413_1211,
        ];
        assert_eq!(shl(&X, 0), X);
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
            shl(&X, 128),
            [0, 0x100F_0E0D_0C0B_0A09_0807_0605_0403_0201,]
        );
        assert_eq!(
            shl(&X, 136),
            [0, 0x0F_0E0D_0C0B_0A09_0807_0605_0403_020100,]
        );
        assert_eq!(
            shl(&X, 248),
            [0, 0x0100_0000_0000_0000_0000_0000_0000_0000,]
        );
        assert_eq!(shl(&X, 256), X);
        assert_eq!(3u128 << 256, 3u128);
    }
}
