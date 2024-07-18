use crate::uint::{
    u128x, u256x::U256, u32x, u512x::{self, U512}
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
        let data11 = u128x::swap32(self.len as u128);
        if len < 511 - 64 {
            data[1][1] |= data11;
            self.hash = compress(self.hash, data);
        } else {
            self.hash = compress(self.hash, data);
            self.hash = compress(self.hash, [[0, 0], [0, data11]]);
        }
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        sha2::{sha224::SHA224, sha256::SHA256},
        uint::{u256x, u512x},
    };

    use super::HashState;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let f = |init, k, len| {
            let state = HashState::new(init);
            state.end(k, len)
        };
        // d14a028c_2a3a2bc9_476102bb_288234c4
        // 15a2b01f_828ea62a_c5b3e42f
        {
            let mut h = f(SHA224, u512x::ZERO, 0);
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
            f(SHA256, u512x::ZERO, 0),
            [
                0x996fb924_9afbf4c8_98fc1c14_e3b0c442,
                0x7852b855_a495991b_649b934c_27ae41e4,
            ],
        );
        // "0"
        // 5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9
        assert_eq!(
            f(SHA256, [[0x3000_0000, 0], [0, 0]], 8),
            u256x::swap32([
                0xc2dbc23_9dd4e91b4_6729d73a_27fb57e9,
                0x5feceb6_6ffc86f38_d952786c_6d696c79,
            ])
        );
        // "01"
        // 938db8c9f82c8cb58d3f3ef4fd250036a48d26a712753d2fde5abd03a85cabf4
        assert_eq!(
            f(SHA256, [[0x3031_0000, 0], [0, 0]], 16),
            u256x::swap32([
                0xa48d26a_712753d2f_de5abd03_a85cabf4,
                0x938db8c_9f82c8cb5_8d3f3ef4_fd250036
            ])
        );
    }
}
