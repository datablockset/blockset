use crate::uint::{
    u256x::{self, U256},
    u512x,
};

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

impl Scalar {
    const ZERO: Self = Self([0, 0]);
    const ONE: Self = Self([1, 0]);
    const MAX: Self = Self(u256x::wsub(P, [1, 0]));
    #[inline(always)]
    const fn new(key: U256) -> Self {
        assert!(is_valid(key));
        Self(key)
    }
    #[inline(always)]
    const fn eq(self, b: Self) -> bool {
        u256x::eq(&self.0, &b.0)
    }
    const fn add(self, b: Self) -> Self {
        let (mut result, o) = u256x::oadd(self.0, b.0);
        if o || !is_valid(result) {
            result = u256x::wsub(result, P)
        }
        Self(result)
    }
    const fn sub(self, b: Self) -> Self {
        let (mut result, b) = u256x::osub(self.0, b.0);
        if b || !is_valid(result) {
            result = u256x::wadd(result, P)
        }
        Self(result)
    }
    #[inline(always)]
    const fn neg(self) -> Self {
        Self::ZERO.sub(self)
    }
    const fn mul(self, b: Self) -> Self {
        Self(u512x::div_rem(u256x::mul(self.0, b.0), [P, u256x::ZERO])[1][0])
    }
    const fn reciprocal2(mut self) -> Vec2 {
        assert!(!Self::ZERO.eq(self));
        let mut a0 = P;
        let mut f0 = [Self::ONE, Self::ZERO];
        let mut f1 = [Self::ZERO, Self::ONE];
        loop {
            if Self::ONE.eq(self) {
                return f1;
            }
            let [q, a2] = u256x::div_rem(a0, self.0);
            a0 = self.0;
            self = Scalar(a2);
            let f2 = sub(f0, mul(f1, Scalar(q)));
            f0 = f1;
            f1 = f2;
        }
    }
    const fn reciprocal(mut self) -> Self {
        assert!(!Self::ZERO.eq(self));
        let mut a0 = P;
        let mut f0 = Self::ZERO;
        let mut f1 = Self::ONE;
        loop {
            if Self::ONE.eq(self) {
                return f1;
            }
            let [q, a2] = u256x::div_rem(a0, self.0);
            a0 = self.0;
            self = Scalar(a2);
            let f2 = f0.sub(f1.mul(Scalar(q)));
            f0 = f1;
            f1 = f2;
        }
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

    use crate::{
        secp256k1::{is_valid_private_key, Scalar, Vec2, P},
        uint::u256x,
    };

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
            Scalar::ZERO
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_mul() {
        assert_eq!(Scalar::ZERO.mul(Scalar::MAX), Scalar::ZERO);
        assert_eq!(Scalar::ONE.mul(Scalar::ONE), Scalar::ONE);
        assert_eq!(
            Scalar::new([2, 0]).mul(Scalar::new([2, 0])),
            Scalar::new([4, 0])
        );
        assert_eq!(Scalar::MAX.mul(Scalar::MAX), Scalar::ONE);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_reciprocal() {
        fn x(s: Scalar) {
            let v = s.reciprocal();
            assert_eq!(v.mul(s), Scalar::ONE);
        }
        fn f(s: Scalar, v: Scalar) {
            assert_eq!(s.reciprocal(), v);
            assert_eq!(v.mul(s), Scalar::ONE);
        }
        f(Scalar::ONE, Scalar::ONE);
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
            assert_eq!(v[1].mul(s), Scalar::ONE);
        }
        fn f(s: Scalar, v: Vec2) {
            assert_eq!(s.reciprocal2(), v);
            assert_eq!(v[1].mul(s), Scalar::ONE);
        }
        f(Scalar::ONE, [Scalar::ZERO, Scalar::ONE]);
        f(Scalar::MAX, [Scalar::ONE, Scalar::MAX]);
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
