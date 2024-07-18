use crate::uint::{u256x::U256, u512x::{self, U512}};

use super::hash_state::HashState;

struct State {
    state: HashState,
    buffer: U512,
    len: u16,
}

impl State {
    const fn new(hash: U256) -> Self {
        Self {
            state: HashState::new(hash),
            buffer: u512x::ZERO,
            len: 0,
        }
    }
    const fn end(self) -> U256 {
        self.state.end(self.buffer, self.len)
    }
}
