use crate::uint::{
    u256x::U256,
    u32x,
    u512x::{self, U512},
};

use super::compress::compress;

pub struct HashState {
    hash: U256,
    len: u64,
}

impl HashState {
    pub const fn new(hash: U256) -> Self {
        Self { hash, len: 0 }
    }
    pub const fn push(self, data: U512) -> Self {
        Self {
            hash: compress(self.hash, data),
            len: self.len + 512,
        }
    }
    pub const fn end(mut self, mut data: U512, len: u16) -> U256 {
        assert!(len < 512);
        {
            let [p, q] = u32x::div_rem(len as u32, 32);
            data = u512x::set_bit(data, p + (31 - q));
        }
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        sha2::{sha224::SHA224, sha256::SHA256},
        uint::u512x,
    };

    use super::HashState;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let f = |init| {
            let state = HashState::new(init);
            state.end(u512x::ZERO, 0)
        };
        // d14a028c_2a3a2bc9_476102bb_288234c4
        // 15a2b01f_828ea62a_c5b3e42f
        {
            let mut h = f(SHA224);
            h[1] |= 0xFFFF_FFFF << 96;
            assert_eq!(
                h,
                [
                    0x288234c4_476102bb_2a3a2bc9_d14a028c,
                    0xFFFFFFFF_c5b3e42f_828ea62a_15a2b01f,
                ]
            );
        }
        // e3b0c442_98fc1c14_9afbf4c8_996fb924
        // 27ae41e4_649b934c_a495991b_7852b855
        assert_eq!(
            f(SHA256),
            [
                0x996fb924_9afbf4c8_98fc1c14_e3b0c442,
                0x7852b855_a495991b_649b934c_27ae41e4,
            ],
        );
    }
}
