use core::panic;

use crate::{
    elliptic_curve::EllipticCurve, field::prime_field_scalar::PrimeFieldScalar, uint::u256x,
};

use super::Order;

pub type Point<C: EllipticCurve> = [PrimeFieldScalar<C>; 2];

const X: usize = 0;
const Y: usize = 1;

const fn eq<C: EllipticCurve>(a: &Point<C>, b: &Point<C>) -> bool {
    a[X].eq(&b[X]) && a[Y].eq(&b[Y])
}

// const _3_DIV_2: Scalar = Scalar::_3.div(Scalar::_2);

const fn from_m<C: EllipticCurve>(
    [x, y]: Point<C>,
    pqx: PrimeFieldScalar<C>,
    m: PrimeFieldScalar<C>,
) -> Point<C> {
    let m2 = m.mul(m);
    let rx = m2.sub(pqx);
    let ry = m.mul(rx.sub(x)).add(y);
    let r = [rx, ry.neg()];
    r
}

const fn neg<C: EllipticCurve>([x, y]: Point<C>) -> Point<C> {
    [x, y.neg()]
}

const fn double<C: EllipticCurve>(p: Point<C>) -> Point<C> {
    let [x, y] = p;
    // if y = 0, it means either the point is `O` or `m` is not defined.
    if y.eq(&PrimeFieldScalar::_0) {
        return PrimeFieldScalar::O;
    }
    from_m(
        p,
        x.mul(PrimeFieldScalar::_2),
        x.mul(x).div(y).mul(PrimeFieldScalar::_3_DIV_2),
    )
}

const fn from_x<C: EllipticCurve>(x: PrimeFieldScalar<C>) -> Point<C> {
    if let Some(y) = x.y() {
        return [x, y];
    }
    panic!();
}

pub const fn add<C: EllipticCurve>(p: Point<C>, q: Point<C>) -> Point<C> {
    let [px, py] = p;
    let [qx, qy] = q;
    if px.eq(&qx) {
        return if py.eq(&qy) {
            double(p)
        } else {
            PrimeFieldScalar::O
        };
    }
    if eq(&p, &PrimeFieldScalar::O) {
        return q;
    }
    if eq(&q, &PrimeFieldScalar::O) {
        return p;
    }
    from_m(p, px.add(qx), py.sub(qy).div(px.sub(qx)))
}

pub const fn mul<C: EllipticCurve>(mut p: Point<C>, mut n: Order<C>) -> Point<C> {
    let mut r = PrimeFieldScalar::O;
    loop {
        if n.0[0] & 1 != 0 {
            r = add(r, p);
        }
        n.0 = u256x::shr(&n.0, 1);
        if u256x::eq(&n.0, &u256x::_0) {
            break;
        }
        p = double(p);
    }
    r
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        elliptic_curve::EllipticCurve,
        secp256k1::{
            point::{from_x, neg},
            scalar::{Scalar, Secp256k1P},
            Order,
        },
    };

    use super::{double, mul, Point};

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
