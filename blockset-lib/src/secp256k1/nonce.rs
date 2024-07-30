use crate::{
    hmac::HmacSha256,
    sha2::be_chunk::BeChunk,
    uint::u256x::{self, U256},
};

use super::field::Field;

pub const fn nonce<const A0: u128, const A1: u128>(pk: U256, m: BeChunk) -> Field<A0, A1> {
    let mut vk = (
        [
            0x01010101_01010101_01010101_01010101,
            0x01010101_01010101_01010101_01010101,
        ],
        u256x::_0,
    );
    const fn g((v, k): (U256, U256)) -> U256 {
        HmacSha256::new([u256x::_0, k])
            .push(&BeChunk::u256(v))
            .end()
    }
    const fn f(pk: U256, m: &BeChunk, mut vk: (U256, U256), s: u8) -> (U256, U256) {
        vk.0 = HmacSha256::new([u256x::_0, vk.0])
            .push(&BeChunk::u256(vk.1))
            .push(&BeChunk::u8(s))
            .push(&BeChunk::u256(pk))
            .push(m)
            .end();
        vk.1 = g(vk);
        vk
    }
    vk = f(pk, &m, vk, 0x00);
    vk = f(pk, &m, vk, 0x01);
    loop {
        vk.1 = g(vk);
        if u256x::less(&vk.1, &Field::<A0, A1>::P) {
            return Field::<A0, A1>::new(vk.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        secp256k1::nonce::nonce,
        sha2::{be_chunk::BeChunk, sha256::SHA256, state::State},
        uint::u256x::{self, U256},
    };

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        const Q: U256 = u256x::be(0x4_00000000, 0x00000000_00020108_A2E0CC0D_99F8A5EF);
        const X: U256 = u256x::be(0x0_9A4D6792, 0x295A7F73_0FC3F2B4_9CBC0F62_E862272F);
        const UX: U256 = u256x::be(0x7_9AEE090D, 0xB05EC252_D5CB4452_F356BE19_8A4FF96F);
        const UY: U256 = u256x::be(0x7_82E29634, 0xDDC9A31E_F40386E8_96BAA18B_53AFA5A3);
        let mut h1 = u256x::swap32(State::new(SHA256).push_array(b"sample").end());
        assert_eq!(
            h1,
            u256x::be(
                0xAF2BDBE1_AA9B6EC1_E2ADE1D6_94F41FC7,
                0x1A831D02_68E98915_62113D8A_62ADD1BF
            )
        );
        const LEN: i32 = 163;
        const I: i32 = 256 - LEN;
        h1 = u256x::shr(&h1, I);
        if !u256x::less(&h1, &Q) {
            h1 = u256x::wsub(h1, Q)
        };
        assert_eq!(
            h1,
            u256x::be(0x01_795EDF0D, 0x54DB760F_156D0DAC_04C0322B_3A204224)
        );
        // h1 = u256x::shl(&h1, I);
        // let _ = nonce::<0x00000000_00020108_A2E0CC0D_99F8A5EF, 0x4_00000000>(u256x::shl(&X, I), BeChunk::new([h1, u256x::_0], LEN as u16));
    }
}
