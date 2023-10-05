#[inline(always)]
pub const fn to_u32x8(&[a, b]: &[u128; 2]) -> [u32; 8] {
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
