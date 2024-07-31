use std::marker::PhantomData;

use crate::uint::u256x::U256;

use super::{elliptic_curve::EllipticCurve, prime::Prime};

pub struct EllipticCurveN<C: EllipticCurve>(PhantomData<C>);

impl<C: EllipticCurve> Prime for EllipticCurveN<C> {
    const P: U256 = C::N;
}
