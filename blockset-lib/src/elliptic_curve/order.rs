use crate::{field::prime_field_scalar::PrimeFieldScalar, nonce::nonce};

use super::{
    order_tag::OrderTag,
    point::{self, Point},
    EllipticCurve,
};

pub type Order<C: EllipticCurve> = PrimeFieldScalar<OrderTag<C>>;
