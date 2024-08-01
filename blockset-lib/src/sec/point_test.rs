#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        elliptic_curve::{
            order::Order,
            point::{double, from_x, mul, neg, Point},
            EllipticCurve,
        },
        prime_field::scalar,
        sec::scalar::Secp256k1P,
    };

    type Scalar = scalar::Scalar<Secp256k1P>;

    const N: Order<Secp256k1P> = Order::unchecked_new(Order::<Secp256k1P>::P);

    #[test]
    #[wasm_bindgen_test]
    fn test_mul_o() {
        assert_eq!(mul(Scalar::O, Order::_0), Scalar::O);
        assert_eq!(mul(Scalar::O, Order::_1), Scalar::O);
        assert_eq!(mul(Scalar::O, Order::n(2)), Scalar::O);
        assert_eq!(mul(Scalar::O, Order::new([0, 1])), Scalar::O);
        assert_eq!(mul(Scalar::O, N), Scalar::O);
    }

    fn check<P: EllipticCurve>([x, y]: Point<P>) {
        assert_eq!(x.y().unwrap().abs(), y.abs());
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_double() {
        let p = from_x(Scalar::_1);
        let p1 = double(p);
        let p2 = double(p1);
        let p3 = double(p2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_mul_1() {
        let g = |p| {
            let pn = neg(p);
            assert_eq!(mul(p, Order::_0), Scalar::O);
            assert_eq!(mul(p, Order::_1), p);
            assert_eq!(mul(p, N), Scalar::O);
            assert_eq!(mul(p, N.sub(Order::_1)), pn);
            //
            let f = |s| {
                let r = mul(p, s);
                check(r);
                let rn = mul(pn, s);
                check(rn);
                assert_ne!(r, Scalar::O);
                assert_ne!(r, p);
                assert_ne!(r, pn);
                assert_ne!(rn, Scalar::O);
                assert_ne!(rn, p);
                assert_ne!(rn, pn);
                assert_ne!(r, rn);
                assert_eq!(r, neg(rn));
            };
            f(Order::n(2));
            f(Order::new([3, 0]));
            f(Order::new([0, 1]));
            f(Order::new([1, 1]));
            f(Order::new([0, 2]));
            f(Order::new([2, 2]));
            f(Order::new([0, 3]));
            f(Order::new([3, 3]));
        };
        let s = |x| g(from_x(x));
        // s(Scalar::_0);
        s(Scalar::_1);
        s(Scalar::_2);
        s(Scalar::_3);
        s(Scalar::n(4));
        // g(Scalar::n(5));
        s(Scalar::n(6));
        // g(Scalar::n(7));
        g(Scalar::G);
    }
}
