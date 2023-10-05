use crate::u32x4::u32x4_add;

pub type U256 = [u128; 2];

#[inline(always)]
pub const fn u32x8_add(&[a0, a1]: &U256, &[b0, b1]: &U256) -> U256 {
    [u32x4_add(a0, b0), u32x4_add(a1, b1)]
}
