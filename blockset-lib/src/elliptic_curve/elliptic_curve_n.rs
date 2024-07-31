use std::marker::PhantomData;

use crate::{field::prime::Prime, uint::u256x::U256};

use super::EllipticCurve;

pub struct EllipticCurveN<C: EllipticCurve>(PhantomData<C>);

impl<C: EllipticCurve> Prime for EllipticCurveN<C> {
    const P: U256 = C::N;
}
