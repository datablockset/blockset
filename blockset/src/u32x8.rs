use crate::u32x4::{to_u128, u32x4_add};

pub type U256 = [u128; 2];

#[inline(always)]
pub const fn to_u32x8(&[a, b]: &U256) -> [u32; 8] {
    [
        a as u32,
        (a >> 32) as u32,
        (a >> 64) as u32,
        (a >> 96) as u32,
        b as u32,
        (b >> 32) as u32,
        (b >> 64) as u32,
        (b >> 96) as u32,
    ]
}

#[inline(always)]
pub const fn to_u256(&[w0, w1, w2, w3, w4, w5, w6, w7]: &[u32; 8]) -> U256 {
    [
        to_u128([w0, w1, w2, w3]),
        to_u128([w4, w5, w6, w7]),
    ]
}

#[inline(always)]
pub const fn u32x8_add(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [
        u32x4_add(a0, b0),
        u32x4_add(a1, b1),
    ]
}