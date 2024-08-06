use crate::uint::u256x::U256;

use super::{be_chunk::BeChunk, hash_state::HashState};

pub struct State {
    state: HashState,
    rest: BeChunk,
}

impl State {
    pub const fn from_hash_state(state: HashState) -> Self {
        Self {
            state,
            rest: BeChunk::default(),
        }
    }
    pub const fn new(hash: U256) -> Self {
        Self::from_hash_state(HashState::new(hash))
    }
    pub const fn end(self) -> U256 {
        self.state.end(self.rest)
    }
    pub const fn push(mut self, rest: &BeChunk) -> Self {
        let (v, rest) = self.rest.chain(rest);
        if let Some(v) = v {
            self.state = self.state.push(v);
        }
        self.rest = rest;
        self
    }
    pub const fn push_array(mut self, v: &[u8]) -> Self {
        let len = v.len();
        let mut i = 0;
        loop {
            if i == len {
                return self;
            }
            self = self.push(&BeChunk::u8(v[i]));
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::sha2::sha256::SHA256;

    use super::State;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let f = |v, a1, a0| {
            let h = State::new(SHA256).push_array(v).end();
            assert_eq!(h, [a0, a1]);
        };
        f(
            b"",
            0xe3b0c442_98fc1c14_9afbf4c8_996fb924,
            0x27ae41e4_649b934c_a495991b_7852b855,
        );
        f(
            b"0",
            0x5feceb6_6ffc86f38_d952786c_6d696c79,
            0xc2dbc23_9dd4e91b4_6729d73a_27fb57e9,
        );
        f(
            b"The quick brown fox jumps over the lazy dog",
            0xd7a8fbb3_07d78094_69ca9abc_b0082e4f,
            0x8d5651e4_6d3cdb76_2d02d0bf_37c9e592,
        );
    }
}
