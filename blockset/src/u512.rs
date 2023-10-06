use crate::u256::U256;

pub type U512 = [U256; 2];

pub const fn new(a: u128, b: u128, c: u128, d: u128) -> U512 {
    [[a, b], [c, d]]
}

pub const fn get_u128(a: &U512, i: usize) -> u128 {
    a[(i >> 1) & 1][i & 1]
}