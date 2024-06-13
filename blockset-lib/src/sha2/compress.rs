use crate::uint::{
    u256x::{u32x8_wadd, U256},
    u512x::U512,
};

use super::{round::round16, w_round::w_round16};

/// https://en.wikipedia.org/wiki/One-way_compression_function
pub const fn compress(init: U256, mut w: U512) -> U256 {
    let mut x = round16(init, &w, 0);
    w = w_round16(w);
    x = round16(x, &w, 1);
    w = w_round16(w);
    x = round16(x, &w, 2);
    w = w_round16(w);
    x = round16(x, &w, 3);
    u32x8_wadd(&x, &init)
}
