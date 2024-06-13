use crate::uint::{
    u128x::{from_u32x4, to_u32x4},
    u32x::add4,
    u512x::{get_u128, U512},
};

use super::sigma32::{SMALL0, SMALL1};

#[inline(always)]
const fn w_round(w0: u32, w1: u32, w9: u32, we: u32) -> u32 {
    add4(SMALL1.get(we), w9, SMALL0.get(w1), w0)
}

#[inline(always)]
const fn wi(w: &U512, i: usize) -> u128 {
    get_u128(w, i)
}

// ```
// | |0   |1   |2   |3   |
// | |0123|0123|0123|0123|
// | |0123|4567|89AB|CDEF|
// |-|----|----|----|----|
// |0|WR  |    | R  |  R |
// |1| WR |    |  R |   R|
// |2|R WR|    |   R|    |
// |3| R W|R   |    |R   |
// ```
//
// - `R` - read
// - `W` - read and write.
const fn w_round4(w: &U512, i: usize) -> u128 {
    let [mut w00, mut w01, mut w02, mut w03] = to_u32x4(wi(w, i));
    let w10 = wi(w, i + 1) as u32;
    let [_, w21, w22, w23] = to_u32x4(wi(w, i + 2));
    let [w30, _, w32, w33] = to_u32x4(wi(w, i + 3));
    w00 = w_round(w00, w01, w21, w32);
    w01 = w_round(w01, w02, w22, w33);
    w02 = w_round(w02, w03, w23, w00);
    w03 = w_round(w03, w10, w30, w01);
    from_u32x4([w00, w01, w02, w03])
}

pub const fn w_round16(mut w: U512) -> U512 {
    w[0][0] = w_round4(&w, 0);
    w[0][1] = w_round4(&w, 1);
    w[1][0] = w_round4(&w, 2);
    w[1][1] = w_round4(&w, 3);
    w
}
