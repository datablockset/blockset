// RFC6979 https://www.rfc-editor.org/rfc/rfc6979

use crate::{
    field::prime::Prime, hmac::HmacSha256, sha2::be_chunk::BeChunk, uint::u256x::{self, U256}
};

use super::field::PrimeField;

struct VK {
    v: U256,
    k: U256,
}

pub const fn nonce<P: Prime>(pk: &U256, m: &U256) -> PrimeField<P> {
    let p = PrimeField::<P>::P;
    let offset = PrimeField::<P>::OFFSET as i32;
    const fn c<P: Prime>(mut v: U256) -> BeChunk {
        let p = PrimeField::<P>::P;
        if !u256x::less(&v, &p) {
            v = u256x::wsub(v, p);
        }
        let offset8 = PrimeField::<P>::OFFSET8;
        BeChunk::new(
            [u256x::_0, u256x::shl(&v, offset8 as i32)],
            256 - offset8 as u16,
        )
    }
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
    const fn s(vk: &VK, b: u8) -> HmacSha256 {
        h(vk).push(&BeChunk::u8(b))
    }
    const fn f(pk: &BeChunk, m: &BeChunk, mut vk: VK, b: u8) -> VK {
        vk.k = s(&vk, b).push(pk).push(m).end();
        vk.v = g(&vk);
        vk
    }
    let pkc = c::<P>(*pk);
    let mc = c::<P>(*m);
    vk = f(&pkc, &mc, vk, 0x00);
    vk = f(&pkc, &mc, vk, 0x01);
    loop {
        vk.v = g(&vk);
        let k = u256x::shr(&vk.v, offset);
        if u256x::less(&k, &p) {
            return PrimeField::<P>::new(k);
        }
        vk.k = s(&vk, 0x00).end();
        vk.v = g(&vk);
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        field::prime::Prime, secp256k1::nonce::nonce, sha2::{sha256::SHA256, state::State}, uint::u256x::{self, U256}
    };

    struct Sect163k1();

    impl Prime for Sect163k1 {
        const P: U256 = u256x::be(0x04_00000000, 0x00000000_00020108_A2E0CC0D_99F8A5EF);
    }

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
        let n = nonce::<Sect163k1>(&X, &h1);
        assert_eq!(
            n.0,
            u256x::be(0x02_3AF4074C, 0x90A02B3F_E61D286D_5C87F425_E6BDD81B)
        );
    }
}
