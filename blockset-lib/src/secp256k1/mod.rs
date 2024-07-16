mod field;
mod point;
mod scalar;

use point::{Point, G};

use crate::uint::u256x::U256;

struct Order(U256);

impl Order {
    const fn public_key(self) -> Point {
        point::mul(G, self.0)
    }
}
