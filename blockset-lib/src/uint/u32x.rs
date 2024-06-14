/// Adds two `u32` integers and returns the sum.
///
/// This function safely handles integer overflow without panicking.
#[inline(always)]
pub const fn wadd(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

/// Adds three `u32` integers and returns the sum.
///
/// This function safely handles integer overflow without panicking.
#[inline(always)]
pub const fn wadd3(a: u32, b: u32, c: u32) -> u32 {
    wadd(wadd(a, b), c)
}

/// Adds four `u32` integers and returns the sum.
///
/// This function safely handles integer overflow without panicking.
#[inline(always)]
pub const fn wadd4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    wadd(wadd(a, b), wadd(c, d))
}

/// Adds five `u32` integers and returns the sum.
///
/// This function safely handles integer overflow without panicking.
#[inline(always)]
pub const fn wadd5(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    wadd3(wadd3(a, b, c), d, e)
}

/// Converts a `u32` integer into an array of four `u8` bytes (little-endian).
#[inline(always)]
pub const fn to_u8x4(a: u32) -> [u8; 4] {
    // [a as u8, (a >> 8) as u8, (a >> 16) as u8, (a >> 24) as u8]
    a.to_le_bytes()
}

/// Constructs a `u32` integer from an array of four `u8` bytes (little-endian).
#[inline(always)]
pub const fn from_u8x4(a: &[u8; 4]) -> u32 {
    // a[0] as u32 | ((a[1] as u32) << 8) | ((a[2] as u32) << 16) | ((a[3] as u32) << 24)
    u32::from_le_bytes(*a)
}

#[cfg(test)]
mod test {
    use super::{to_u8x4, wadd, wadd3, wadd4, wadd5};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        assert_eq!(wadd(1, 2), 3);
        assert_eq!(wadd3(1, 2, 3), 6);
        assert_eq!(wadd4(1, 2, 3, 4), 10);
        assert_eq!(wadd5(1, 2, 3, 4, 5), 15);
        assert_eq!(to_u8x4(0x12345678), [0x78, 0x56, 0x34, 0x12]);
    }
}
