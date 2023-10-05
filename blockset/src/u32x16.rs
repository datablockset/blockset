#[inline(always)]
pub const fn to_u32x16(&[a, b, c, d]: &[u128; 4]) -> [u32; 16] {
    [
        a as u32,
        (a >> 32) as u32,
        (a >> 64) as u32,
        (a >> 96) as u32,
        b as u32,
        (b >> 32) as u32,
        (b >> 64) as u32,
        (b >> 96) as u32,
        c as u32,
        (c >> 32) as u32,
        (c >> 64) as u32,
        (c >> 96) as u32,
        d as u32,
        (d >> 32) as u32,
        (d >> 64) as u32,
        (d >> 96) as u32,
    ]
}
