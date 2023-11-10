use crate::uint::{
    u128::{get_u32, to_u32x4},
    u256::U256,
    u32::{add, add4},
    u512::{get_u128, new, U512},
};

use super::sigma32::{BIG0, BIG1};

const fn round([s0, s1]: U256, i: usize, w: u128, k: u128) -> U256 {
    let (a, e) = {
        let t1 = {
            let [e, f, g, h] = to_u32x4(s1);
            add4(
                h,
                BIG1.get(e),
                (e & f) ^ (!e & g),
                get_u32(k, i),
                get_u32(w, i),
            )
        };
        let [a, b, c, d] = to_u32x4(s0);
        let t2 = add(BIG0.get(a), (a & b) ^ (a & c) ^ (b & c));
        (add(t1, t2), add(d, t1))
    };
    [a as u128 | (s0 << 32), e as u128 | (s1 << 32)]
}

const fn round4(mut x: U256, i: usize, w: &U512, k: &U512) -> U256 {
    let w = get_u128(w, i);
    let k = get_u128(k, i);
    x = round(x, 0, w, k);
    x = round(x, 1, w, k);
    x = round(x, 2, w, k);
    round(x, 3, w, k)
}

pub const K: [U512; 4] = [
    new(
        0xe9b5dba5_b5c0fbcf_71374491_428a2f98,
        0xab1c5ed5_923f82a4_59f111f1_3956c25b,
        0x550c7dc3_243185be_12835b01_d807aa98,
        0xc19bf174_9bdc06a7_80deb1fe_72be5d74,
    ),
    new(
        0x240ca1cc_0fc19dc6_efbe4786_e49b69c1,
        0x76f988da_5cb0a9dc_4a7484aa_2de92c6f,
        0xbf597fc7_b00327c8_a831c66d_983e5152,
        0x14292967_06ca6351_d5a79147_c6e00bf3,
    ),
    new(
        0x53380d13_4d2c6dfc_2e1b2138_27b70a85,
        0x92722c85_81c2c92e_766a0abb_650a7354,
        0xc76c51a3_c24b8b70_a81a664b_a2bfe8a1,
        0x106aa070_f40e3585_d6990624_d192e819,
    ),
    new(
        0x34b0bcb5_2748774c_1e376c08_19a4c116,
        0x682e6ff3_5b9cca4f_4ed8aa4a_391c0cb3,
        0x8cc70208_84c87814_78a5636f_748f82ee,
        0xc67178f2_bef9a3f7_a4506ceb_90befffa,
    ),
];

pub const fn round16(mut x: U256, w: &U512, i: usize) -> U256 {
    let k = &K[i];
    x = round4(x, 0, w, k);
    x = round4(x, 1, w, k);
    x = round4(x, 2, w, k);
    round4(x, 3, w, k)
}
