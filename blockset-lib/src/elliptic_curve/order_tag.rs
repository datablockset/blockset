use std::marker::PhantomData;

use crate::{field::prime::Prime, uint::u256x::U256};

use super::EllipticCurve;

pub struct OrderTag<C: EllipticCurve>(PhantomData<C>);

impl<C: EllipticCurve> Prime for OrderTag<C> {
    const P: U256 = C::N;
}
