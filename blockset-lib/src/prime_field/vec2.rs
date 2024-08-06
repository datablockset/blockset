use super::{prime::Prime, scalar::Scalar};

pub type Vec2<P> = [Scalar<P>; 2];

pub const fn mul<P: Prime>([x, y]: Vec2<P>, a: Scalar<P>) -> Vec2<P> {
    [x.mul(a), y.mul(a)]
}

pub const fn sub<P: Prime>([x0, y0]: Vec2<P>, [x1, y1]: Vec2<P>) -> Vec2<P> {
    [x0.sub(x1), y0.sub(y1)]
}
