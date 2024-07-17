use crate::{
    sha2::{self, compress::compress, sha256::SHA256},
    uint::u256x::{self, U256},
};

const fn repeat(v: u128) -> U256 {
    [v, v]
}

const I_PAD: U256 = repeat(0x36363636_36363636_36363636_36363636);
const O_PAD: U256 = repeat(0x5C5C5C5C_5C5C5C5C_5C5C5C5C_5C5C5C5C);

const fn hmac(key: U256, msg: U256) -> U256 {
    let hash = compress(SHA256, [u256x::bitor(&key, &I_PAD), msg]);
    compress(SHA256, [u256x::bitor(&key, &O_PAD), hash])
}
