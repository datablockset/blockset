use crate::uint::u256x::U256;

use super::prime::Prime;

pub trait EllipticCurve: Prime {
    const GX: U256;
    const GY: U256;
    const A: U256;
    const B: U256;
    const N: U256;
}
