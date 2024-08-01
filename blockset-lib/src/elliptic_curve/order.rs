use crate::{field::prime_field_scalar::PrimeFieldScalar, nonce::nonce};

use super::{
    order_tag::OrderTag,
    point::{self, Point},
    EllipticCurve,
};

pub type Order<C: EllipticCurve> = PrimeFieldScalar<OrderTag<C>>;

type Signature<C: EllipticCurve> = [PrimeFieldScalar<OrderTag<C>>; 2];

impl<C: EllipticCurve> Order<C> {
    pub const fn public_key(self) -> Point<C> {
        point::mul(PrimeFieldScalar::G, self)
    }
    pub const fn sign(self, z: Self) -> Signature<C> {
        let k = nonce(&self.0, &z.0);
        let r = Self::new(point::mul(PrimeFieldScalar::G, k)[0].0);
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
    let p =
        Order::new(point::add(point::mul(PrimeFieldScalar::G, u1), point::mul(pub_key, u2))[0].0);
    p.eq(&r)
}
