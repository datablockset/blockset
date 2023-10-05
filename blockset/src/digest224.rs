pub type Digest224 = [u32; 7];

pub fn parity_bit(d: &Digest224) -> u8 {
    d.iter().fold(0, |a, b| a ^ b.count_ones()) as u8 & 1
}

#[cfg(test)]
mod tests {
    use super::parity_bit;

    #[test]
    fn test() {
        assert_eq!(parity_bit(&[0; 7]), 0);
        assert_eq!(parity_bit(&[0xFFFF_FFFF; 7]), 0);
        assert_eq!(parity_bit(&[0x8000_0000; 7]), 1);
        //                      0  1  1  2  1  2  2 =  9
        assert_eq!(parity_bit(&[0, 1, 2, 3, 4, 5, 6]), 1);
    }
}
