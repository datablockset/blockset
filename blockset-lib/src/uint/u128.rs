use crate::uint::u32::add;

#[inline(always)]
pub const fn to_u32x4(v: u128) -> [u32; 4] {
    [
        v as u32,
        (v >> 32) as u32,
        (v >> 64) as u32,
        (v >> 96) as u32,
    ]
}

#[inline(always)]
pub const fn from_u32x4([w0, w1, w2, w3]: [u32; 4]) -> u128 {
    w0 as u128 | ((w1 as u128) << 32) | ((w2 as u128) << 64) | ((w3 as u128) << 96)
}

#[inline(always)]
pub const fn get_u32(v: u128, i: usize) -> u32 {
    (v >> (i << 5)) as u32
}

#[inline(always)]
pub const fn u32x4_add(a: u128, b: u128) -> u128 {
    let [a0, a1, a2, a3] = to_u32x4(a);
    let [b0, b1, b2, b3] = to_u32x4(b);
    from_u32x4([add(a0, b0), add(a1, b1), add(a2, b2), add(a3, b3)])
}
