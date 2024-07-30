use crate::{
    hmac::HmacSha256,
    sha2::be_chunk::BeChunk,
    uint::u256x::{self, U256},
};

use super::field::Field;

pub const fn nonce<const A0: u128, const A1: u128>(pk: U256, m: U256) -> Field<A0, A1> {
    let mut vk = (
        [
            0x01010101_01010101_01010101_01010101,
            0x01010101_01010101_01010101_01010101,
        ],
        u256x::_0,
    );
    const fn g((v, k): (U256, U256)) -> U256 {
        HmacSha256::new([u256x::_0, k]).push(BeChunk::u256(v)).end()
    }
    const fn f(pk: U256, m: U256, mut vk: (U256, U256), s: u8) -> (U256, U256) {
        vk.0 = HmacSha256::new([u256x::_0, vk.0])
            .push(BeChunk::u256(vk.1))
            .push(BeChunk::u8(s))
            .push(BeChunk::u256(pk))
            .push(BeChunk::u256(m))
            .end();
        vk.1 = g(vk);
        vk
    }
    vk = f(pk, m, vk, 0x00);
    vk = f(pk, m, vk, 0x01);
    loop {
        vk.1 = g(vk);
        if u256x::less(&vk.1, &Field::<A0, A1>::P) {
            return Field::<A0, A1>::new(vk.1);
        }
    }
}
