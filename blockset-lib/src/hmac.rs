use crate::{
    sha2::{hash_state::HashState, sha256::SHA256},
    uint::{
        u256x::{self, U256},
        u512x::{self, U512},
    },
};

const fn repeat(v: u128) -> U256 {
    [v, v]
}

const I_PAD: U256 = repeat(0x36363636_36363636_36363636_36363636);
const O_PAD: U256 = repeat(0x5C5C5C5C_5C5C5C5C_5C5C5C5C_5C5C5C5C);

const fn sha256(data: U512) -> U256 {
    let mut state = HashState::new(SHA256);
    state = state.push(data);
    state.end(u512x::ZERO, 0)
}

const fn op(key: U256, pad: U256, data: U256) -> U256 {
    sha256([u256x::bitor(&key, &pad), data])
}

pub const fn hmac(key: U256, msg: U256) -> U256 {
    let hash = op(key, I_PAD, msg);
    op(key, O_PAD, hash)
}
