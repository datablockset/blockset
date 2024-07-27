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

struct State {
    state: sha2::state::State,
    key: U512,
}

impl State {
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
        Self::hash_state(self.key, O_PAD).end([u256x::ZERO, u256x::swap32(self.state.end())], 0x100)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::uint::{u256x, u512x};

    use super::State;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        assert_eq!(
            State::new(u512x::ZERO).end(),
            u256x::swap32([
                0xff1697c4_93715653_c6c71214_4292c5ad,
                0xb613679a_0814d9ec_772f95d7_78c35fc5,
            ])
        );
    }
}
