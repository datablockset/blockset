#[inline(always)]
pub const fn div_rem(a: u64, b: u64) -> (u64, u64) {
    (a / b, a % b)
}
