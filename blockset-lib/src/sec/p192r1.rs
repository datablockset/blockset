use crate::{
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

/*
impl EllipticCurve for P256k1 {
    const GX: U256 = u256x::be(
        0x79BE667E_F9DCBBAC_55A06295_CE870B07,
        0x029BFCDB_2DCE28D9_59F2815B_16F81798,
    );
    const GY: U256 = u256x::be(
        0x483ADA77_26A3C465_5DA4FBFC_0E1108A8,
        0xFD17B448_A6855419_9C47D08F_FB10D4B8,
    );
    const A: U256 = u256x::_0;
    const B: U256 = u256x::from_u128(7);
    const N: U256 = u256x::be(
        0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE,
        0xBAAEDCE6_AF48A03B_BFD25E8C_D0364141,
    );
}
*/
