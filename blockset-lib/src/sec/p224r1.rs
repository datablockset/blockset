// https://neuromancer.sk/std/secg/secp224r1

use crate::{
    elliptic_curve::EllipticCurve,
    prime_field::prime::Prime,
    uint::u256x::{self, U256},
};

pub struct P224r1();

impl Prime for P224r1 {
    const P: U256 = u256x::be(
        0x00000000_fffffff_ffffffff_fffffffff,
        0xffffffff_00000000_00000000_00000001,
    );
}

impl EllipticCurve for P224r1 {
    const GX: U256 = u256x::be(
        0x00000000_b70e0cbd_6bb4bf7f_321390b9,
        0x4a03c1d3_56c21122_343280d6_115c1d21,
    );
    const GY: U256 = u256x::be(
        0x00000000_bd376388_b5f723fb_4c22dfe6,
        0xcd4375a0_5a074764_44d58199_85007e34,
    );
    const A: U256 = u256x::be(
        0x00000000_ffffffff_ffffffff_ffffffff,
        0xfffffffe_ffffffff_ffffffff_fffffffe,
    );
    const B: U256 = u256x::be(
        0x00000000_b4050a85_0c04b3ab_f5413256,
        0x5044b0b7_d7bfd8ba_270b3943_2355ffb4,
    );
    const N: U256 = u256x::be(
        0x00000000_ffffffff_ffffffff_ffffffff,
        0xffff16a2_e0b8f03e_13dd2945_5c5c2a3d,
    );
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::sec::test::gen_test1;

    use super::P224r1;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        gen_test1::<P224r1>();
    }
}
