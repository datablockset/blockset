mod field;
mod point;
mod scalar;

use field::Field;
use point::{Point, G};

type Order = Field<0xBAAEDCE6_AF48A03B_BFD25E8C_D0364141, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE>;

type Signature = [Order; 2];

impl Order {
    const fn public_key(self) -> Point {
        point::mul(G, self)
    }
    const fn sign(self, z: Order, k: Order) -> Signature {
        let r = Order::new(point::mul(G, k)[0].0);
        let s = z.add(r.mul(self)).div(k);
        [r, s]
    }
}

const fn verify(pub_key: Point, z: Order, [r, s]: Signature) -> bool {
    let si = s.reciprocal();
    let u1 = z.mul(si);
    let u2 = r.mul(si);
    let p = Order::new(point::add(point::mul(G, u1), point::mul(pub_key, u2))[0].0);
    p.eq(&r)
}
