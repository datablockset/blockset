use crate::u256::U256;

// It should work a little bit faster than (a ^ b).leading_zeros().
pub const fn dif(&[a0, a1]: &U256, &[b0, b1]: &U256) -> usize {
    let mut result = 0;
    let mut v = a0 ^ b0;
    if v == 0 {
        v = a1 ^ b1;
        result += 128;
    }
    result + (v.leading_zeros() as usize)
}

#[cfg(test)]
mod test {
    use super::dif;

    #[test]
    fn test() {
        assert_eq!(dif(&[0, 0], &[0, 0]), 256);
        assert_eq!(dif(&[0, 0], &[0, 1]), 255);
        assert_eq!(dif(&[1, 0], &[1, 0]), 256);
        assert_eq!(dif(&[0, 0], &[1, 458]), 127);
        assert_eq!(dif(&[0b111, 0], &[0b100, 458]), 126);
        assert_eq!(
            dif(&[0, 0], &[0x80000000_00000000_00000000_00000000, 458]),
            0
        );
    }
}
