pub mod order;
pub mod order_tag;
pub mod point;
pub mod scalar;

use crate::{prime_field::prime::Prime, uint::u256x::U256};

pub trait EllipticCurve: Prime {
    const GX: U256;
    const GY: U256;
    const A: U256;
    const B: U256;
    const N: U256;
}
