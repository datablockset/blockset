// RFC6979 https://www.rfc-editor.org/rfc/rfc6979

use crate::{
    hmac::HmacSha256,
    sha2::be_chunk::BeChunk,
    uint::u256x::{self, U256},
};

use super::field::Field;

struct VK {
    v: U256,
    k: U256,
}

pub const fn nonce<const A0: u128, const A1: u128>(pk: &BeChunk, m: &BeChunk) -> Field<A0, A1> {
    let mut vk = VK {
        v: [
            0x01010101_01010101_01010101_01010101,
            0x01010101_01010101_01010101_01010101,
        ],
        k: u256x::_0,
    };
    const fn h(&VK { v, k }: &VK) -> HmacSha256 {
        HmacSha256::new([u256x::_0, k]).push(&BeChunk::u256(v))
    }
    const fn g(vk: &VK) -> U256 {
        h(vk).end()
    }
    const fn f(pk: &BeChunk, m: &BeChunk, mut vk: VK, s: u8) -> VK {
        vk.k = h(&vk).push(&BeChunk::u8(s)).push(pk).push(m).end();
        vk.v = g(&vk);
        vk
    }
    vk = f(pk, m, vk, 0x00);
    vk = f(pk, m, vk, 0x01);
    let p = Field::<A0, A1>::P;
    let s = u256x::leading_zeros(p) as i32;
    loop {
        vk.v = g(&vk);
        let k = u256x::shr(&vk.v, s);
        if u256x::less(&k, &p) {
            return Field::<A0, A1>::new(k);
        }
        vk.k = h(&vk).push(&BeChunk::u8(0x00)).end();
        vk.v = g(&vk);
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        secp256k1::nonce::nonce,
        sha2::{be_chunk::BeChunk, sha256::SHA256, state::State},
        uint::{
            u256x::{self, U256},
            u512x,
        },
    };

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        const Q: U256 = u256x::be(0x04_00000000, 0x00000000_00020108_A2E0CC0D_99F8A5EF);
        const X: U256 = u256x::be(0x00_9A4D6792, 0x295A7F73_0FC3F2B4_9CBC0F62_E862272F);
        const UX: U256 = u256x::be(0x07_9AEE090D, 0xB05EC252_D5CB4452_F356BE19_8A4FF96F);
        const UY: U256 = u256x::be(0x07_82E29634, 0xDDC9A31E_F40386E8_96BAA18B_53AFA5A3);
        let mut h1 = State::new(SHA256).push_array(b"sample").end();
        assert_eq!(
            h1,
            u256x::be(
                0xAF2BDBE1_AA9B6EC1_E2ADE1D6_94F41FC7,
                0x1A831D02_68E98915_62113D8A_62ADD1BF
            )
        );
        const LEN: u16 = 163;
        const I: i32 = 256 - LEN as i32;
        h1 = u256x::shr(&h1, I);
        if !u256x::less(&h1, &Q) {
            h1 = u256x::wsub(h1, Q)
        };
        assert_eq!(
            h1,
            u256x::be(0x01_795EDF0D, 0x54DB760F_156D0DAC_04C0322B_3A204224)
        );
        const LEN8: u16 = ((LEN + 7) >> 3) << 3;
        let chunk = |x| BeChunk::new(u512x::shl(&[x, u256x::_0], 512 - LEN8 as i32), LEN8);
        let n = nonce::<0x00000000_00020108_A2E0CC0D_99F8A5EF, 0x4_00000000>(&chunk(X), &chunk(h1));
        assert_eq!(
            n.0,
            u256x::be(0x02_3AF4074C, 0x90A02B3F_E61D286D_5C87F425_E6BDD81B)
        );
    }
}
