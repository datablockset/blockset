use crate::u128::{to_u32x4, u32x4_add};

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
