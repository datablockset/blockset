use core::panic;

use crate::uint::u256x::{self, U256};

use super::scalar::Scalar;

type Point = [Scalar; 2];

const X: usize = 0;
const Y: usize = 1;

// [0, 0] is not on a curve so we can use it as an infinity point.
// even more, when x = 0, y is not defined.
const O: Point = [Scalar::_0, Scalar::_0];

const fn eq(a: Point, b: Point) -> bool {
    a[X].eq(b[X]) && a[Y].eq(b[Y])
}

const _3_DIV_2: Scalar = Scalar::_3.div(Scalar::_2);

const fn from_m([x, y]: Point, pqx: Scalar, m: Scalar) -> Point {
    let m2 = m.mul(m);
    let rx = m2.sub(pqx);
    let ry = m.mul(rx.sub(x)).add(y);
    let r = [rx, ry.neg()];
    r
}

const fn neg([x, y]: Point) -> Point {
    [x, y.neg()]
}

const fn double(p: Point) -> Point {
    let [x, y] = p;
    // if y = 0, it means either the point is `O` or `m` is not defined.
    if y.eq(Scalar::_0) {
        return O
    }
    from_m(p, x.mul(Scalar::_2), x.mul(x).div(y).mul(_3_DIV_2))
}

const fn from_x(x: Scalar) -> Point {
    if let Some(y) = x.y() {
        return [x, y];
    }
    panic!();
}

const fn add(p: Point, q: Point) -> Point {
    let [px, py] = p;
    let [qx, qy] = q;
    if px.eq(qx) {
        return if py.eq(qy) { double(p) } else { O }
    }
    if eq(p, O) {
        return q;
    }
    if eq(q, O) {
        return p;
    }
    from_m(p, px.add(qx), py.sub(qy).div(px.sub(qx)))
}

const fn mul(mut p: Point, mut n: U256) -> Point {
    let mut r = O;
    loop {
        if n[0] & 1 != 0 {
            r = add(r, p);
        }
        n = u256x::shr(&n, 1);
        if u256x::eq(&n, &u256x::ZERO) {
            break
        }
        p = double(p);
    }
    r
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{secp256k1::{point::{from_x, neg, O, Y}, scalar::{Scalar, N}}, uint::u256x};

    use super::{double, mul, Point, X};

    #[test]
    #[wasm_bindgen_test]
    fn test_mul_o() {
        assert_eq!(mul(O, [0, 0]), O);
        assert_eq!(mul(O, [1, 0]), O);
        assert_eq!(mul(O, [2, 0]), O);
        assert_eq!(mul(O, [0, 1]), O);
        assert_eq!(mul(O, N), O);
    }

    fn check([x, y]: Point) {
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
        let g = |x| {
            let p = from_x(x);
            let pn = neg(p);
            assert_eq!(mul(p, [0, 0]), O);
            assert_eq!(mul(p, [1, 0]), p);
            assert_eq!(mul(p, N), O);
            assert_eq!(mul(p, u256x::wsub(N, [1, 0])), pn);
            //
            let f = |s| {
                let r = mul(p, s);
                check(r);
                let rn = mul(pn, s);
                check(rn);
                assert_ne!(r, O);
                assert_ne!(r, p);
                assert_ne!(r, pn);
                assert_ne!(rn, O);
                assert_ne!(rn, p);
                assert_ne!(rn, pn);
                assert_ne!(r, rn);
                assert_eq!(r, neg(rn));
            };
            f([2, 0]);
            f([3, 0]);
            f([0, 1]);
            f([1, 1]);
            f([0, 2]);
            f([2, 2]);
            f([0, 3]);
            f([3, 3]);
        };
        // g(Scalar::_0);
        g(Scalar::_1);
        g(Scalar::_2);
        g(Scalar::_3);
        g(Scalar::n(4));
        // g(Scalar::n(5));
        g(Scalar::n(6));
        // g(Scalar::n(7));
    }
}