use crate::prime_field::scalar::Scalar;

use super::order_tag::OrderTag;

pub type Order<C> = Scalar<OrderTag<C>>;
