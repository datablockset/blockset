#![cfg(test)]

use crate::{elliptic_curve::EllipticCurve, prime_field::scalar::Scalar, uint::u256x};

fn sqrt_test<C: EllipticCurve>(c: Scalar<C>) {
    let c2 = c.mul(c);
    let s = c2.sqrt().unwrap();
    assert_eq!(c, s.abs());
}

fn pow_common<C: EllipticCurve>(s: Scalar<C>) {
    assert_eq!(s.pow(Scalar::_0), Scalar::_1);
    assert_eq!(s.pow(Scalar::_1), s);
    // https://en.wikipedia.org/wiki/Fermat%27s_little_theorem
    // a^(p-1) % p = 1
    assert_eq!(s.pow(Scalar::MIDDLE).abs(), Scalar::_1);
    assert_eq!(s.pow(Scalar::MAX.sub(Scalar::_1)), s.reciprocal());
    assert_eq!(s.pow(Scalar::MAX), Scalar::_1);
}

fn pow_test<C: EllipticCurve>() {
    let s2 = Scalar::<C>::new([2, 0]);
    let s3 = Scalar::<C>::n(3);
    let s4 = Scalar::<C>::new([4, 0]);
    let s8 = Scalar::<C>::new([8, 0]);
    let s9 = Scalar::<C>::new([9, 0]);
    let s27 = Scalar::<C>::new([27, 0]);
    // 0
    assert_eq!(Scalar::<C>::_0.pow(Scalar::_0), Scalar::_1);
    assert_eq!(Scalar::<C>::_0.pow(Scalar::MAX), Scalar::_0);
    // 1
    pow_common(Scalar::<C>::_1);
    // 2
    pow_common(s2);
    assert_eq!(s2.pow(s2), s4);
    assert_eq!(s2.pow(s3), s8);
    assert_eq!(s2.pow(Scalar::new([128, 0])), Scalar::new([0, 1]));
    assert_eq!(
        s2.pow(Scalar::new([191, 0])),
        Scalar::new([0, 0x0000_0000_0000_0000_8000_0000_0000_0000])
    );
    // 3
    pow_common(s3);
    assert_eq!(s3.pow(s2), s9);
    assert_eq!(s3.pow(Scalar::n(3)), s27);
    assert_eq!(
        s3.pow(Scalar::new([100, 0])),
        Scalar::new(u256x::be(0x5a4653ca, 0x67376856_5b41f775_d6947d55_cf3813d1))
    );
    assert_eq!(
        s3.pow(Scalar::new([110, 0])),
        Scalar::new(u256x::be(
            0x5156_c7b52959,
            0xeb5176ff_82e03b94_10a1d5d2_2dd2daf9
        ))
    );
    assert_eq!(
        s3.pow(Scalar::new([120, 0])),
        Scalar::new(u256x::be(
            0x4949a9b6_99bf15c7,
            0x89b11e42_db8e5bb0_60f0fceb_b0ee4461
        ))
    );
    assert_eq!(
        s3.pow(Scalar::new([121, 0])),
        Scalar::new(u256x::be(
            0xdbdcfd23_cd3d4156,
            0x9d135ac8_92ab1311_22d2f6c3_12cacd23
        ))
    );
    // Gx
    pow_common(Scalar::<C>::G[0]);
    // MIDDLE
    pow_common(Scalar::<C>::MIDDLE);
    // MAX-1
    pow_common(Scalar::<C>::MAX.sub(Scalar::_1));
    // MAX
    pow_common(Scalar::<C>::MAX);
}

pub fn gen_test<C: EllipticCurve>() {
    assert_eq!(Scalar::<C>::G[0].y2(), Scalar::G[1].mul(Scalar::G[1]));
    assert_eq!(Scalar::<C>::G[0].y().unwrap(), Scalar::G[1]);
    // SQRT
    for i in 1..1000 {
        sqrt_test(Scalar::<C>::new([i, 0]));
    }
    sqrt_test(Scalar::<C>::G[0]);
    sqrt_test(Scalar::<C>::MIDDLE);
    // pow
    pow_test::<C>();
}
