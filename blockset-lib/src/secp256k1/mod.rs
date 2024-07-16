mod field;
mod point;
mod scalar;

use field::Field;
use point::{Point, G};

type Order = Field<0xBAAEDCE6_AF48A03B_BFD25E8C_D0364141, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE>;

impl Order {
    const fn public_key(self) -> Point {
        point::mul(G, self)
    }
}
