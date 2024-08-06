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
mod p256r1;
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
        elliptic_curve::{order::Order, EllipticCurve},
        nonce::nonce,
        prime_field::scalar::Scalar,
        sec::{p224r1::P224r1, p256k1::P256k1, verify},
        sha2::{sha256::SHA256, state::State},
        uint::u256x::{self, U256},
    };

    use super::p192r1::P192r1;

    #[test]
    #[wasm_bindgen_test]
    fn test_192() {
        fn g<C: EllipticCurve>(
            xe: U256,
            px: U256,
            py: U256,
            k0: U256,
            r0: U256,
            s0: U256,
            k1: U256,
            r1: U256,
            s1: U256,
        ) {
            let x = Order::<C>::new(xe);
            let public_key = x.public_key();
            assert_eq!(public_key, [Scalar::new(px), Scalar::new(py)]);
            let f = |a, ke, re, se| {
                let h1 = Scalar::from_be(State::new(SHA256).push_array(a).end());
                let k = nonce(&x, &h1);
                assert_eq!(k.0, ke);
                let [r, s] = x.sign(h1);
                assert_eq!(r.0, re);
                assert_eq!(s.0, se);
            };
            f(b"sample", k0, r0, s0);
            f(b"test", k1, r1, s1);
        }
        g::<P192r1>(
            u256x::be(0x6FAB0349_34E4C0FC, 0x9AE67F5B_5659A9D7_D1FEFD18_7EE09FD4),
            u256x::be(0xAC2C77F5_29F91689, 0xFEA0EA5E_FEC7F210_D8EEA0B9_E047ED56),
            u256x::be(0x3BC723E5_7670BD48, 0x87EBC732_C523063D_0A7C957B_C97C1C43),
            u256x::be(0x32B1B6D7_D42A05CB, 0x44906572_7A84804F_B1A3E34D_8F261496),
            u256x::be(0x4B0B8CE9_8A92866A, 0x2820E20A_A6B75B56_382E0F9B_FD5ECB55),
            u256x::be(0xCCDB0069_26EA9565, 0xCBADC840_829D8C38_4E06DE1F_1E381B85),
            u256x::be(0x5C4CE89C_F56D9E7C, 0x77C85853_39B006B9_7B5F0680_B4306C6C),
            u256x::be(0x3A718BD8_B4926C3B, 0x52EE6BBE_67EF79B1_8CB6EB62_B1AD97AE),
            u256x::be(0x5662E684_8A4A19B1, 0xF1AE2F72_ACD4B8BB_E50F1EAC_65D9124F),
        );
        g::<P224r1>(
            u256x::be(
                0xF220266E_1105BFE3_083E03EC,
                0x7A3A6546_51F45E37_167E8860_0BF257C1,
            ),
            u256x::be(
                0x00CF08DA_5AD719E4_2707FA43,
                0x1292DEA1_1244D64F_C51610D9_4B130D6C,
            ),
            u256x::be(
                0xEEAB6F3D_EBE455E3_DBF85416,
                0xF7030CBD_94F34F2D_6F232C69_F3C1385A,
            ),
            u256x::be(
                0xAD3029E0_278F8064_3DE33917,
                0xCE6908C7_0A8FF50A_411F06E4_1DEDFCDC,
            ),
            u256x::be(
                0x61AA3DA0_10E8E840_6C656BC4,
                0x77A7A718_9895E7E8_40CDFE8F_F42307BA,
            ),
            u256x::be(
                0xBC814050_DAB5D237_70879494,
                0xF9E0A680_DC1AF716_1991BDE6_92B10101,
            ),
            u256x::be(
                0xFF86F579_24DA248D_6E44E815,
                0x4EB69F0A_E2AEBAEE_9931D0B5_A969F904,
            ),
            u256x::be(
                0xAD04DDE8_7B84747A_243A631E,
                0xA47A1BA6_D1FAA059_149AD244_0DE6FBA6,
            ),
            u256x::be(
                0x178D49B1_AE90E3D8_B629BE3D,
                0xB5683915_F4E8C99F_DF6E666C_F37ADCFD,
            ),
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
