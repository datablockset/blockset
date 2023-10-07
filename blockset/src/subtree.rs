use crate::u256::U256;

pub const fn dif(&[a0, a1]: &U256, &[b0, b1]: &U256) -> usize {
    let mut result = 0;
    let mut v = a0 ^ b0;
    if v == 0 {
        v = a1 ^ b1;
        if v == 0 {
            return 256;
        }
        result += 128;
    }
    result + (v.leading_zeros() as usize)
}

#[cfg(test)]
mod test {
    use super::dif;

    #[test]
    fn test() {
        //assert_eq!(0u32.leading_zeros(), 32);
        // assert_eq!(dif(&[0, 0], &[0, 0]), 256);
    }
}