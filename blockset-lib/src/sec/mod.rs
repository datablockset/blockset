use crate::{
    elliptic_curve::{
        order::Order,
        point::{self, Point},
        EllipticCurve,
    },
    nonce::nonce,
    prime_field::scalar::Scalar,
};

mod p192r1;
mod p224r1;
mod p256k1;
mod test;

type Signature<C> = [Order<C>; 2];

impl<C: EllipticCurve> Order<C> {
    pub const fn public_key(self) -> Point<C> {
        point::mul(Scalar::G, self)
    }
    pub const fn sign(self, z: Self) -> Signature<C> {
        let k = nonce(&self, &z);
        let r = Self::new(point::mul(Scalar::G, k)[0].0);
        let s = z.add(r.mul(self)).div(k);
        [r, s]
    }
}

pub const fn verify<C: EllipticCurve>(
    pub_key: Point<C>,
    z: Order<C>,
    [r, s]: Signature<C>,
) -> bool {
    let si = s.reciprocal();
    let u1 = z.mul(si);
    let u2 = r.mul(si);
    let p = Order::new(point::add(point::mul(Scalar::G, u1), point::mul(pub_key, u2))[0].0);
    p.eq(&r)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        elliptic_curve::order::Order,
        nonce::nonce,
        prime_field::scalar::Scalar,
        sec::{p256k1::P256k1, verify},
        sha2::{sha256::SHA256, state::State},
        uint::u256x,
    };

    use super::p192r1::P192r1;

    #[test]
    #[wasm_bindgen_test]
    fn test_192() {
        let x = Order::<P192r1>::new(u256x::be(
            0x6FAB0349_34E4C0FC,
            0x9AE67F5B_5659A9D7_D1FEFD18_7EE09FD4,
        ));
        let public_key = x.public_key();
        assert_eq!(
            public_key,
            [
                Scalar::new(u256x::be(
                    0xAC2C77F5_29F91689,
                    0xFEA0EA5E_FEC7F210_D8EEA0B9_E047ED56
                )),
                Scalar::new(u256x::be(
                    0x3BC723E5_7670BD48,
                    0x87EBC732C523063D0A7C957BC97C1C43
                ))
            ]
        );
        let f = |a, ke, re, se| {
            let h1 = Scalar::from_be(State::new(SHA256).push_array(a).end());
            let k = nonce(&x, &h1);
            assert_eq!(k.0, ke);
            let [r, s] = x.sign(h1);
            assert_eq!(r.0, re);
            assert_eq!(s.0, se);
        };
        f(
            b"sample",
            u256x::be(0x32B1B6D7_D42A05CB, 0x44906572_7A84804F_B1A3E34D_8F261496),
            u256x::be(0x4B0B8CE9_8A92866A, 0x2820E20A_A6B75B56_382E0F9B_FD5ECB55),
            u256x::be(0xCCDB0069_26EA9565, 0xCBADC840_829D8C38_4E06DE1F_1E381B85),
        );
        f(
            b"test",
            u256x::be(0x5C4CE89C_F56D9E7C, 0x77C85853_39B006B9_7B5F0680_B4306C6C),
            u256x::be(0x3A718BD8_B4926C3B, 0x52EE6BBE_67EF79B1_8CB6EB62_B1AD97AE),
            u256x::be(0x5662E684_8A4A19B1, 0xF1AE2F72_ACD4B8BB_E50F1EAC_65D9124F),
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let f = |p, h| {
            // Alice
            let private_key = Order::<P256k1>::new(p);
            let public_key = private_key.public_key();
            let hash = Order::new(h);
            let signature = private_key.sign(hash);
            // Bob
            let result = verify(public_key, hash, signature);
            assert!(result);
            // Enemy
            let w_private_key = private_key.add(Order::_1);
            let w_signature = w_private_key.sign(hash);
            // Bob
            let w_result = verify(public_key, hash, w_signature);
            assert!(!w_result);
        };
        f(
            [
                1234567890_1234567890_1234567890_123456789,
                234567890_1234567890_1234567890_1234567890,
            ],
            [
                34567890_1234567890_1234567890_1234567890,
                4567890_1234567890_1234567890_1234567890_1,
            ],
        );
        f(
            [
                7890_1234567890_1234567890_1234567890_1234,
                890_1234567890_1234567890_1234567890_12345,
            ],
            [
                90_1234567890_1234567890_1234567890_123456,
                1234567890_1234567890_1234567890_123456790,
            ],
        );
        f(
            [
                1111111111_2222222222_3333333333_444444444,
                4444444444_5555555555_6666666666_77777777,
            ],
            [
                8888888888_9999999999_0000000000_11111111,
                2222222222_3333333333_4444444444_555555555,
            ],
        );
        f([u128::MAX, u128::MAX], [u128::MAX, u128::MAX]);
    }
}
