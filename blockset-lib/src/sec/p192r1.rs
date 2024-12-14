use crate::{
    elliptic_curve::EllipticCurve,
    prime_field::prime::Prime,
    uint::u256x::{self, U256},
};

// https://neuromancer.sk/std/secg/secp192r1
pub struct P192r1();

impl Prime for P192r1 {
    const P: U256 = u256x::be(
        0x00000000_00000000_ffffffff_ffffffff,
        0xffffffff_fffffffe_ffffffff_ffffffff,
    );
}

impl EllipticCurve for P192r1 {
    const GX: U256 = u256x::be(
        0x00000000_00000000_188da80e_b03090f6,
        0x7cbf20eb_43a18800_f4ff0afd_82ff1012,
    );
    const GY: U256 = u256x::be(
        0x00000000_00000000_07192b95_ffc8da78,
        0x631011ed_6b24cdd5_73f977a1_1e794811,
    );
    const A: U256 = u256x::be(
        0x00000000_00000000_ffffffff_ffffffff,
        0xffffffff_fffffffe_ffffffff_fffffffc,
    );
    const B: U256 = u256x::be(
        0x00000000_00000000_64210519_e59c80e7,
        0x0fa7e9ab_72243049_feb8deec_c146b9b1,
    );
    const N: U256 = u256x::be(
        0x00000000_00000000_ffffffff_ffffffff,
        0xffffffff_99def836_146bc9b1_b4d22831,
    );
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{prime_field::scalar::Scalar, sec::test::gen_test};

    use super::P192r1;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        gen_test::<P192r1>();
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_b() {
        let r0 = Scalar::<P192r1>::B.sqrt().unwrap();
        let r1 = Scalar::<P192r1>::_0.y().unwrap();
        assert_eq!(r0.abs(), r1.abs());
    }
}
