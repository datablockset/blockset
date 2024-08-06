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
        prime_field::scalar::Scalar,
        sec::{p256k1::P256k1, verify},
        uint::u256x,
    };

    use super::p192r1::P192r1;

    #[test]
    #[wasm_bindgen_test]
    fn test_x() {
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
