use crate::{
    sha2::{self, hash_state::HashState, sha256::SHA256},
    uint::{
        u256x::{self, U256},
        u512x::{self, U512},
    },
};

const fn repeat(v: u128) -> U512 {
    let x = [v, v];
    [x, x]
}

const I_PAD: U512 = repeat(0x36363636_36363636_36363636_36363636);
const O_PAD: U512 = repeat(0x5C5C5C5C_5C5C5C5C_5C5C5C5C_5C5C5C5C);

struct HmacSha256 {
    state: sha2::state::State,
    key: U512,
}

impl HmacSha256 {
    const fn hash_state(key: U512, pad: U512) -> HashState {
        HashState::new(SHA256).push(u512x::bitxor(&key, &pad))
    }
    const fn new(key: U512) -> Self {
        Self {
            state: Self::hash_state(key, I_PAD).state(),
            key,
        }
    }
    #[inline(always)]
    const fn push_array(mut self, a: &[u8]) -> Self {
        self.state = self.state.push_array(a);
        self
    }
    const fn end(self) -> U256 {
        Self::hash_state(self.key, O_PAD).end([u256x::_0, u256x::swap32(self.state.end())], 0x100)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::uint::{u256x, u512x};

    use super::HmacSha256;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        assert_eq!(
            HmacSha256::new(u512x::_0).end(),
            u256x::swap32([
                0xff1697c4_93715653_c6c71214_4292c5ad,
                0xb613679a_0814d9ec_772f95d7_78c35fc5,
            ])
        );
        assert_eq!(
            HmacSha256::new(u512x::be(
                0x1000_0000_0000_0000_0000_0000_0000_0000,
                0,
                0,
                0
            ))
            .end(),
            u256x::swap32([
                0x720f729a_884cf655_581a0f6b_83e05d01,
                0xe5ed5d2c_d3d2da2c_8a23322c_a509cc41
            ])
        );
        assert_eq!(
            HmacSha256::new(u512x::be(
                0x1000_0000_0000_0000_0000_0000_0000_0000,
                0,
                0,
                0
            ))
            .push_array(b"The quick brown fox jumps over the lazy dog")
            .end(),
            u256x::swap32([
                0x3f900df9_52fcd88a_d4dc6134_a2b7af12,
                0x0b8e8977_aa8b1ad0_5691c746_04ed9cf6
            ])
        );
        assert_eq!(
            HmacSha256::new(u512x::be(
                0xFFFF_FFFF_0000_0000_0000_0000_0000_0000,
                0,
                0,
                0
            ))
            .push_array(b"The quick brown fox jumps over the lazy dog")
            .end(),
            u256x::swap32([
                0xdcf8a948_20e1e8eb_0010e0b2_14b89fa8,
                0x8d0e6c9d_de99251d_18203a1a_c3288a93,
            ])
        )
    }
}
