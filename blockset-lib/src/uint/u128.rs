use crate::uint::u32::add;

/// Converts a 128-bit unsigned integer (`u128`) into a vector of four 32-bit unsigned integers (`[u32; 4]`).
/// This function essentially 'splits' the 128-bit value into a vector of four components, each representing a
/// 32-bit segment, starting from the least significant bits.
#[inline(always)]
pub const fn to_u32x4(v: u128) -> [u32; 4] {
    [
        v as u32,
        (v >> 32) as u32,
        (v >> 64) as u32,
        (v >> 96) as u32,
    ]
}

/// Reconstructs a 128-bit unsigned integer (`u128`) from a vector of four 32-bit unsigned integers (`[u32; 4]`).
/// This operation is the inverse of `to_u32x4`, combining the vector components back into a single 128-bit value.
#[inline(always)]
pub const fn from_u32x4([w0, w1, w2, w3]: [u32; 4]) -> u128 {
    w0 as u128 | ((w1 as u128) << 32) | ((w2 as u128) << 64) | ((w3 as u128) << 96)
}

/// Extracts a single 32-bit component (element) from a 128-bit vector (`u128`) at a specified index.
/// The index `i` determines which 32-bit segment to extract, with `0` being the least significant.
#[inline(always)]
pub const fn get_u32(v: u128, i: usize) -> u32 {
    (v >> (i << 5)) as u32
}

/// Performs element-wise addition of two 128-bit vectors (`u128`), represented as arrays of 32-bit components.
/// Each component of the vectors is added using `add`, which handles overflow by wrapping around.
#[inline(always)]
pub const fn u32x4_add(a: u128, b: u128) -> u128 {
    let [a0, a1, a2, a3] = to_u32x4(a);
    let [b0, b1, b2, b3] = to_u32x4(b);
    from_u32x4([add(a0, b0), add(a1, b1), add(a2, b2), add(a3, b3)])
}

#[inline(always)]
pub const fn shl(u: u128, i: i32) -> u128 {
    match i {
        -127..=-1 => u >> -i,
        0..=127 => u << i,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::uint::u128::shl;

    #[wasm_bindgen_test]
    #[test]
    fn shl_test() {
        assert_eq!(shl(1, -130), 0);
    }
}
