#[inline(always)]
pub const fn add(a: u32, b: u32) -> u32 {
    a.overflowing_add(b).0
}

#[inline(always)]
pub const fn add2(a: u32, b: u32, c: u32) -> u32 {
    add(add(a, b), c)
}

#[inline(always)]
pub const fn add3(a: u32, b: u32, c: u32, d: u32) -> u32 {
    add(add(a, b), add(c, d))
}

#[inline(always)]
pub const fn add4(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    add2(add2(a, b, c), d, e)
}