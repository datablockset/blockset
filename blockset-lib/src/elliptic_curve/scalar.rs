use crate::{
    field::{prime::Prime, prime_field_scalar::PrimeFieldScalar},
    uint::u256x,
};

use super::EllipticCurve;

impl<P: EllipticCurve> PrimeFieldScalar<P> {
    pub const _2: Self = Self::n(2);
    pub const _3: Self = Self::n(3);
    pub const _3_DIV_2: Self = Self::_3.div(Self::_2);
    pub const A: Self = Self::new(P::A);
    pub const B: Self = Self::new(P::B);
    // Points:
    pub const G: [Self; 2] = [Self::new(P::GX), Self::new(P::GY)];
    // Note: [0, 0] should not be on a curve so we can use it as an infinity point.
    // `b != 0`.
    pub const O: [Self; 2] = [Self::_0, Self::_0];
    //
    pub const fn y2(self) -> Self {
        self.pow(Self::_3).add(self.mul(Self::A)).add(Self::B)
    }
    pub const fn y(self) -> Option<Self> {
        self.y2().sqrt()
    }
}
