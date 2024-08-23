// https://neuromancer.sk/std/secg/secp256r1

use crate::{
    elliptic_curve::EllipticCurve,
    prime_field::prime::Prime,
    uint::u256x::{self, U256},
};

pub struct P256r1();

impl Prime for P256r1 {
    const P: U256 = u256x::be(
        0xffffffff_00000001_00000000_00000000,
        0x00000000_ffffffff_ffffffff_ffffffff,
    );
}

impl EllipticCurve for P256r1 {
    const GX: U256 = u256x::be(
        0x6b17d1f2_e12c4247_f8bce6e5_63a440f2,
        0x77037d81_2deb33a0_f4a13945_d898c296,
    );
    const GY: U256 = u256x::be(
        0x4fe342e2_fe1a7f9b_8ee7eb4a_7c0f9e16,
        0x2bce3357_6b315ece_cbb64068_37bf51f5,
    );
    const A: U256 = u256x::be(
        0xffffffff_00000001_00000000_00000000,
        0x00000000_ffffffff_ffffffff_fffffffc,
    );
    const B: U256 = u256x::be(
        0x5ac635d8_aa3a93e7_b3ebbd55_769886bc,
        0x651d06b0_cc53b0f6_3bce3c3e_27d2604b,
    );
    const N: U256 = u256x::be(
        0xffffffff_00000000_ffffffff_ffffffff,
        0xbce6faad_a7179e84_f3b9cac2_fc632551,
    );
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::sec::test::gen_test;

    use super::P256r1;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        gen_test::<P256r1>();
    }
}
