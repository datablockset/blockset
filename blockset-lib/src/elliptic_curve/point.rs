use crate::{field::prime_field_scalar::PrimeFieldScalar, uint::u256x};

use super::{order::Order, EllipticCurve};

pub type Point<C: EllipticCurve> = [PrimeFieldScalar<C>; 2];

const X: usize = 0;
const Y: usize = 1;

const fn eq<C: EllipticCurve>(a: &Point<C>, b: &Point<C>) -> bool {
    a[X].eq(&b[X]) && a[Y].eq(&b[Y])
}

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

pub const fn neg<C: EllipticCurve>([x, y]: Point<C>) -> Point<C> {
    [x, y.neg()]
}

pub const fn double<C: EllipticCurve>(p: Point<C>) -> Point<C> {
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

pub const fn from_x<C: EllipticCurve>(x: PrimeFieldScalar<C>) -> Point<C> {
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
