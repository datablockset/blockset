use crate::{
    elliptic_curve::EllipticCurve,
    prime_field::prime::Prime,
    uint::u256x::{self, U256},
};

// https://en.bitcoin.it/wiki/Secp256k1
// https://neuromancer.sk/std/secg/secp256k1
pub struct P256k1();

impl Prime for P256k1 {
    const P: U256 = u256x::be(
        0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
        0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
    );
}

impl EllipticCurve for P256k1 {
    const GX: U256 = u256x::be(
        0x79BE667E_F9DCBBAC_55A06295_CE870B07,
        0x029BFCDB_2DCE28D9_59F2815B_16F81798,
    );
    const GY: U256 = u256x::be(
        0x483ADA77_26A3C465_5DA4FBFC_0E1108A8,
        0xFD17B448_A6855419_9C47D08F_FB10D4B8,
    );
    const A: U256 = u256x::_0;
    const B: U256 = u256x::from_u128(7);
    const N: U256 = u256x::be(
        0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE,
        0xBAAEDCE6_AF48A03B_BFD25E8C_D0364141,
    );
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        elliptic_curve::{order::Order, point::from_x},
        prime_field,
        sec::{
            p256k1::P256k1,
            test::{gen_test, gen_test_double, test_point_mul},
        },
        uint::u256x::{self, U256},
    };

    type Scalar = prime_field::scalar::Scalar<P256k1>;

    const fn is_valid(key: U256) -> bool {
        u256x::less(&key, &Scalar::P)
    }

    const fn is_valid_private_key(key: U256) -> bool {
        u256x::less(&u256x::_0, &key) && is_valid(key)
    }

    const Q2: Scalar = Scalar::new([
        25454351255596125846892804522787951607,
        43929286026618122883815740552890121610,
    ]);

    #[test]
    #[wasm_bindgen_test]
    fn test_y() {
        gen_test::<P256k1>();
        fn check(lo: u128, hi: u128, some: bool) {
            assert_eq!(Scalar::new([lo, hi]).y().is_some(), some);
        }
        // some random numbers
        check(
            0x9fd69639_62398010_7c2f54a3_5a168569,
            0xc288ac7d_64d0e032_1978d304_cce41ac9,
            true,
        );
        check(
            0xdee33137_a71b5674_78700202_824cc0b4,
            0x0c7e1456_b02e4892_ae84b0d8_fbc104f6,
            true,
        );
        check(
            0xf8278695_19ceeb05_02738f13_0ee52287,
            0x604c5652_7ee62bb6_925b7286_69e67659,
            false,
        );
        //
        assert_eq!(Scalar::_1.y().unwrap(), Q2.mul(Scalar::n(2)));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_sqrt() {
        assert_eq!(Scalar::n(2).sqrt(), Some(Q2));
        assert_eq!(Scalar::n(3).sqrt(), None);
        assert_eq!(Scalar::n(5).sqrt(), None);
        assert_eq!(Scalar::n(6).sqrt(), None);
        // So $y^2 = x^3 + 7$ is not defined when $x = 0$.
        assert_eq!(Scalar::n(7).sqrt(), None);
        assert_eq!(Scalar::new([8, 0]).sqrt(), Some(Q2.mul(Scalar::n(2))));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_pow() {
        assert_eq!(
            Scalar::_2.pow(Scalar::new([255, 0])),
            Scalar::new([0, 0x8000_0000_0000_0000_0000_0000_0000_0000])
        );
        assert_eq!(
            Scalar::_3.pow(Scalar::new([122, 0])),
            Scalar::new(u256x::be(
                0x2_9396f76b_67b7c403,
                0xd73a1059_b8013933_6878e449_38606769
            ))
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        //
        assert!(!is_valid_private_key([0, 0]));
        assert!(is_valid_private_key([1, 0]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0
        ]));
        //
        assert!(is_valid_private_key([0, 1]));
        assert!(is_valid_private_key([1, 1]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            1
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            1
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            1
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(!is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(!is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_add() {
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .add(Scalar::new([1, 0])),
            Scalar::new([0, 0])
        );
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .add(Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])),
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2D,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
        );
    }
    #[test]
    #[wasm_bindgen_test]
    fn test_sub() {
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .sub(Scalar::new([1, 0])),
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2D,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
        );
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .sub(Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2D,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])),
            Scalar::new([1, 0])
        );
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2D,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .sub(Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])),
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
        );
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .sub(Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])),
            Scalar::_0
        );
    }

    const N: Order<P256k1> = Order::unchecked_new(Order::<P256k1>::P);

    #[test]
    #[wasm_bindgen_test]
    fn test_double() {
        gen_test_double(Scalar::_1);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_mul_1() {
        let s = |x| test_point_mul(from_x(x));
        s(Scalar::_1);
        s(Scalar::_2);
        s(Scalar::_3);
        s(Scalar::n(4));
        s(Scalar::n(6));
    }
}
