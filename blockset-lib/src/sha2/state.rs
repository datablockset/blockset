use crate::uint::{
    u256x::U256,
    u512x::{self, U512},
};

use super::compress::compress;

pub struct State {
    hash: U256,
    len: u64,
}

impl State {
    const fn new(hash: U256) -> Self {
        Self { hash, len: 0 }
    }
    const fn push(self, data: U512) -> Self {
        Self {
            hash: compress(self.hash, data),
            len: self.len + 512,
        }
    }
    const fn end(mut self, mut data: U512, len: u16) -> U256 {
        assert!(len < 512);
        data = u512x::set_bit(data, 511 - len as u32);
        self.len += len as u64;
        if len < 511 - 64 {
            data[0][0] |= self.len as u128;
            self.hash = compress(self.hash, data);
        } else {
            self.hash = compress(self.hash, data);
            self.hash = compress(self.hash, [[self.len as u128, 0], [0, 0]]);
        }
        self.hash
    }
}
