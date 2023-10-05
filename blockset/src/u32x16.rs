use crate::u32x4::to_u128;

pub type U512 = [u128; 4];

#[inline(always)]
pub const fn to_u32x16(&[a, b, c, d]: &U512) -> [u32; 16] {
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

#[inline(always)]
pub const fn to_u512(
    &[w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, wa, wb, wc, wd, we, wf]: &[u32; 16],
) -> U512 {
    [
        to_u128([w0, w1, w2, w3]),
        to_u128([w4, w5, w6, w7]),
        to_u128([w8, w9, wa, wb]),
        to_u128([wc, wd, we, wf]),
    ]
}
