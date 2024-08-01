use crate::{
    elliptic_curve::EllipticCurve,
    field::{prime::Prime, prime_field_scalar::PrimeFieldScalar},
    uint::u256x::{self, U256},
};

pub struct Secp256k1P();

impl Prime for Secp256k1P {
    const P: U256 = u256x::be(
        0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
        0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
    );
}

impl EllipticCurve for Secp256k1P {
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

// https://en.bitcoin.it/wiki/Secp256k1
pub type Scalar = PrimeFieldScalar<Secp256k1P>;

const B: U256 = u256x::from_u128(7);

struct Compressed {
    parity: bool,
    x: U256,
}

struct Uncompressed {
    x: U256,
    y: U256,
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        field::vec2::Vec2,
        sec::scalar::Secp256k1P,
        uint::u256x::{self, U256},
    };

    use super::Scalar;

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
        assert_eq!(Scalar::G[0].y2(), Scalar::G[1].mul(Scalar::G[1]));
        assert_eq!(Scalar::G[0].y().unwrap(), Scalar::G[1]);
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
        assert_eq!(Scalar::_1.sqrt(), Some(Scalar::_1));
        assert_eq!(Scalar::n(2).sqrt(), Some(Q2));
        assert_eq!(Scalar::n(3).sqrt(), None);
        assert_eq!(Scalar::n(4).sqrt(), Some(Scalar::n(2)));
        assert_eq!(Scalar::n(5).sqrt(), None);
        assert_eq!(Scalar::n(6).sqrt(), None);
        // So $y^2 = x^3 + 7$ is not defined when $x = 0$.
        assert_eq!(Scalar::n(7).sqrt(), None);
        assert_eq!(Scalar::new([8, 0]).sqrt(), Some(Q2.mul(Scalar::n(2))));
        assert_eq!(Scalar::n(9).sqrt(), Some(Scalar::n(3).neg()));
        assert_eq!(Scalar::new([16, 0]).sqrt(), Some(Scalar::new([4, 0])));
        assert_eq!(Scalar::new([25, 0]).sqrt(), Some(Scalar::new([5, 0]).neg()));
        assert_eq!(Scalar::new([36, 0]).sqrt(), Some(Scalar::new([6, 0]).neg()));
        assert_eq!(Scalar::new([49, 0]).sqrt(), Some(Scalar::new([7, 0]).neg()));
        assert_eq!(Scalar::new([64, 0]).sqrt(), Some(Scalar::new([8, 0])));
        assert_eq!(Scalar::new([81, 0]).sqrt(), Some(Scalar::new([9, 0])));
        assert_eq!(
            Scalar::new([100, 0]).sqrt(),
            Some(Scalar::new([10, 0]).neg())
        );
        assert_eq!(Scalar::new([121, 0]).sqrt(), Some(Scalar::new([11, 0])));
        assert_eq!(
            Scalar::new([144, 0]).sqrt(),
            Some(Scalar::new([12, 0]).neg())
        );
        assert_eq!(
            Scalar::new([169, 0]).sqrt(),
            Some(Scalar::new([13, 0]).neg())
        );
        assert_eq!(
            Scalar::new([196, 0]).sqrt(),
            Some(Scalar::new([14, 0]).neg())
        );
        assert_eq!(Scalar::new([225, 0]).sqrt(), Some(Scalar::new([15, 0])));
        fn check(c: Scalar) {
            let c2 = c.mul(c);
            let s = c2.sqrt().unwrap();
            assert_eq!(c, s.abs());
        }
        for i in 1..1000 {
            check(Scalar::new([i, 0]));
        }
        check(Scalar::G[0]);
        check(Scalar::MIDDLE);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_pow() {
        let s2 = Scalar::new([2, 0]);
        let s3 = Scalar::n(3);
        let s4 = Scalar::new([4, 0]);
        let s8 = Scalar::new([8, 0]);
        let s9 = Scalar::new([9, 0]);
        let s27 = Scalar::new([27, 0]);
        const MAX_S1: Scalar = Scalar::MAX.sub(Scalar::_1);
        fn common(s: Scalar) {
            assert_eq!(s.pow(Scalar::_0), Scalar::_1);
            assert_eq!(s.pow(Scalar::_1), s);
            // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
            // a^(p-1) % p = 1
            assert_eq!(s.pow(Scalar::MIDDLE).abs(), Scalar::_1);
            assert_eq!(s.pow(MAX_S1), s.reciprocal());
            assert_eq!(s.pow(Scalar::MAX), Scalar::_1);
        }
        // 0
        assert_eq!(Scalar::_0.pow(Scalar::_0), Scalar::_1);
        assert_eq!(Scalar::_0.pow(Scalar::MAX), Scalar::_0);
        // 1
        common(Scalar::_1);
        // 2
        common(s2);
        assert_eq!(s2.pow(s2), s4);
        assert_eq!(s2.pow(s3), s8);
        assert_eq!(s2.pow(Scalar::new([128, 0])), Scalar::new([0, 1]));
        assert_eq!(
            s2.pow(Scalar::new([255, 0])),
            Scalar::new([0, 0x8000_0000_0000_0000_0000_0000_0000_0000])
        );
        // 3
        common(Scalar::n(3));
        assert_eq!(Scalar::n(3).pow(s2), s9);
        assert_eq!(Scalar::n(3).pow(Scalar::n(3)), s27);
        // Gx
        common(Scalar::G[0]);
        // MIDDLE
        common(Scalar::MIDDLE);
        // MAX-1
        common(MAX_S1);
        // MAX
        common(Scalar::MAX);
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

    #[test]
    #[wasm_bindgen_test]
    fn test_mul() {
        assert_eq!(Scalar::_0.mul(Scalar::MAX), Scalar::_0);
        assert_eq!(Scalar::_1.mul(Scalar::_1), Scalar::_1);
        assert_eq!(
            Scalar::new([2, 0]).mul(Scalar::new([2, 0])),
            Scalar::new([4, 0])
        );
        assert_eq!(Scalar::MAX.mul(Scalar::MAX), Scalar::_1);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_reciprocal() {
        fn x(s: Scalar) {
            let v = s.reciprocal();
            assert_eq!(v.mul(s), Scalar::_1);
        }
        fn f(s: Scalar, v: Scalar) {
            assert_eq!(s.reciprocal(), v);
            assert_eq!(v.mul(s), Scalar::_1);
        }
        f(Scalar::_1, Scalar::_1);
        f(Scalar::MAX, Scalar::MAX);
        x(Scalar::new([2, 0]));
        x(Scalar::new([3, 0]));
        x(Scalar::new([4, 0]));
        x(Scalar::new([u128::MAX, 0]));
        x(Scalar::new([5, 1]));
        x(Scalar::new([u128::MAX, 1]));
        x(Scalar::new([6, 2]));
        x(Scalar::new([7, 3]));
        x(Scalar::new([8, u128::MAX]));
        x(Scalar::new([Scalar::P[0] - 9, u128::MAX]));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_reciprocal2() {
        fn x(s: Scalar) {
            let v = s.reciprocal2();
            assert_eq!(v[1].mul(s), Scalar::_1);
        }
        fn f(s: Scalar, v: Vec2<Secp256k1P>) {
            assert_eq!(s.reciprocal2(), v);
            assert_eq!(v[1].mul(s), Scalar::_1);
        }
        f(Scalar::_1, [Scalar::_0, Scalar::_1]);
        f(Scalar::MAX, [Scalar::_1, Scalar::MAX]);
        x(Scalar::new([2, 0]));
        x(Scalar::new([3, 0]));
        x(Scalar::new([4, 0]));
        x(Scalar::new([u128::MAX, 0]));
        x(Scalar::new([5, 1]));
        x(Scalar::new([u128::MAX, 1]));
        x(Scalar::new([6, 2]));
        x(Scalar::new([7, 3]));
        x(Scalar::new([8, u128::MAX]));
        x(Scalar::new([Scalar::P[0] - 9, u128::MAX]));
    }
}
