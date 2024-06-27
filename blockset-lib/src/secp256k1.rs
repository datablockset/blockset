use crate::uint::{
    u256x::{self, U256},
    u512x,
};

// https://en.bitcoin.it/wiki/Secp256k1
const P: U256 = [
    0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
    0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
];

const fn is_valid(key: U256) -> bool {
    u256x::less(&key, &P)
}

const fn is_valid_private_key(key: U256) -> bool {
    u256x::less(&u256x::ZERO, &key) && is_valid(key)
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Scalar(U256);

const N: U256 = [0xBAAEDCE6_AF48A03B_BFD25E8C_D0364141, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE];

impl Scalar {
    const _0: Self = Self::n(0);
    const _1: Self = Self::n(1);
    const _3: Self = Self::n(3);
    const _7: Self = Self::n(7);
    const MAX: Self = Self::new(u256x::wsub(P, [1, 0]));
    const MIDDLE: Scalar = Self::new(u256x::shr(&P, 1));
    // (P+1)/4
    const SQRT_K: Scalar = Scalar::new(u256x::shr(&u256x::wadd(P, [1, 0]), 2));
    // Gx
    const GX: Scalar = Scalar::new([
        0x029BFCDB_2DCE28D9_59F2815B_16F81798,
        0x79BE667E_F9DCBBAC_55A06295_CE870B07,
    ]);
    const GY: Scalar = Scalar::new([
        0xFD17B448_A6855419_9C47D08F_FB10D4B8,
        0x483ADA77_26A3C465_5DA4FBFC_0E1108A8,
    ]);
    #[inline(always)]
    const fn new(num: U256) -> Self {
        assert!(is_valid(num));
        Self(num)
    }
    #[inline(always)]
    const fn n(num: u128) -> Self {
        Self::new([num, 0])
    }
    #[inline(always)]
    const fn eq(self, b: Self) -> bool {
        u256x::eq(&self.0, &b.0)
    }
    const fn add(self, b: Self) -> Self {
        self.sub(b.neg())
    }
    const fn sub(self, b: Self) -> Self {
        let (mut result, b) = u256x::osub(self.0, b.0);
        if b {
            result = u256x::wadd(result, P)
        }
        Self(result)
    }
    #[inline(always)]
    const fn neg(self) -> Self {
        Self::_0.sub(self)
    }
    #[inline(always)]
    const fn is_neg(self) -> bool {
        u256x::less(&Self::MIDDLE.0, &self.0)
    }
    #[inline(always)]
    const fn abs(self) -> Self {
        if self.is_neg() {
            self.neg()
        } else {
            self
        }
    }
    const fn mul(self, b: Self) -> Self {
        Self(u512x::div_rem(u256x::mul(self.0, b.0), [P, u256x::ZERO])[1][0])
    }
    const fn reciprocal2(mut self) -> Vec2 {
        assert!(!Self::_0.eq(self));
        let mut a0 = P;
        let mut f0 = [Self::_1, Self::_0];
        let mut f1 = [Self::_0, Self::_1];
        loop {
            if Self::_1.eq(self) {
                return f1;
            }
            let [q, a2] = u256x::div_rem(a0, self.0);
            a0 = self.0;
            self = Self(a2);
            let f2 = sub(f0, mul(f1, Self(q)));
            f0 = f1;
            f1 = f2;
        }
    }
    const fn reciprocal(mut self) -> Self {
        assert!(!Self::_0.eq(self));
        let mut a0 = P;
        let mut f0 = Self::_0;
        let mut f1 = Self::_1;
        loop {
            if Self::_1.eq(self) {
                return f1;
            }
            let [q, a2] = u256x::div_rem(a0, self.0);
            a0 = self.0;
            self = Self(a2);
            let f2 = f0.sub(f1.mul(Self(q)));
            f0 = f1;
            f1 = f2;
        }
    }
    const fn pow(mut self, mut n: Self) -> Self {
        let mut result = Self::_1;
        while !Self::_0.eq(n) {
            if n.0[0] & 1 == 1 {
                result = result.mul(self);
            }
            self = self.mul(self);
            n.0 = u256x::shr(&n.0, 1);
        }
        result
    }
    const fn sqrt(self) -> Option<Self> {
        let result = self.pow(Self::SQRT_K);
        if result.mul(result).eq(self) {
            Some(result)
        } else {
            None
        }
    }
    const fn y2(self) -> Self {
        self.pow(Self::_3).add(Self::_7)
    }
    const fn y(self) -> Option<Self> {
        self.y2().sqrt()
    }
}

type Vec2 = [Scalar; 2];

const fn mul([x, y]: Vec2, a: Scalar) -> Vec2 {
    [x.mul(a), y.mul(a)]
}

const fn sub([x0, y0]: Vec2, [x1, y1]: Vec2) -> Vec2 {
    [x0.sub(x1), y0.sub(y1)]
}

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

    use crate::secp256k1::{is_valid_private_key, Scalar, Vec2, P};

    const Q2: Scalar = Scalar::new([
        25454351255596125846892804522787951607,
        43929286026618122883815740552890121610,
    ]);

    #[test]
    #[wasm_bindgen_test]
    fn test_y() {
        assert_eq!(Scalar::GX.y2(), Scalar::GY.mul(Scalar::GY));
        assert_eq!(Scalar::GX.y().unwrap(), Scalar::GY);
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
        assert_eq!(Scalar::n(7).sqrt(), None);
        assert_eq!(
            Scalar::new([8, 0]).sqrt(),
            Some(Q2.mul(Scalar::n(2)))
        );
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
        check(Scalar::GX);
        check(Scalar::MIDDLE);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_pow() {
        let s2 = Scalar::new([2, 0]);
        let s3 = Scalar::_3;
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
        common(Scalar::_3);
        assert_eq!(Scalar::_3.pow(s2), s9);
        assert_eq!(Scalar::_3.pow(Scalar::_3), s27);
        // Gx
        common(Scalar::GX);
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
        x(Scalar::new([P[0] - 9, u128::MAX]));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_reciprocal2() {
        fn x(s: Scalar) {
            let v = s.reciprocal2();
            assert_eq!(v[1].mul(s), Scalar::_1);
        }
        fn f(s: Scalar, v: Vec2) {
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
        x(Scalar::new([P[0] - 9, u128::MAX]));
    }
}
