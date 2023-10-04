pub type Digest224 = [u32; 7];

pub const fn eq(
    [a0, a1, a2, a3, a4, a5, a6]: Digest224,
    [b0, b1, b2, b3, b4, b5, b6]: Digest224,
) -> bool {
    a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3 && a4 == b4 && a5 == b5 && a6 == b6
}
