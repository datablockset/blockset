use crate::{nonce::nonce, prime_field::scalar::Scalar};

use super::{
    order_tag::OrderTag,
    point::{self, Point},
    EllipticCurve,
};

pub type Order<C: EllipticCurve> = Scalar<OrderTag<C>>;
