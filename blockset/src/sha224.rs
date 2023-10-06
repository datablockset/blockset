use crate::{
    overflow32::{add, add3, add4},
    sigma32::{BIG0, BIG1, SMALL0, SMALL1},
    u128::{from_u32x4, get_u32, to_u32x4},
    u256::{u32x8_add, U256},
    u512::U512,
};

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
    let w = w[i];
    let k = k[i];
    x = round(x, 0, w, k);
    x = round(x, 1, w, k);
    x = round(x, 2, w, k);
    round(x, 3, w, k)
}

pub const K: [U512; 4] = [
    [
        0xe9b5dba5_b5c0fbcf_71374491_428a2f98,
        0xab1c5ed5_923f82a4_59f111f1_3956c25b,
        0x550c7dc3_243185be_12835b01_d807aa98,
        0xc19bf174_9bdc06a7_80deb1fe_72be5d74,
    ],
    [
        0x240ca1cc_0fc19dc6_efbe4786_e49b69c1,
        0x76f988da_5cb0a9dc_4a7484aa_2de92c6f,
        0xbf597fc7_b00327c8_a831c66d_983e5152,
        0x14292967_06ca6351_d5a79147_c6e00bf3,
    ],
    [
        0x53380d13_4d2c6dfc_2e1b2138_27b70a85,
        0x92722c85_81c2c92e_766a0abb_650a7354,
        0xc76c51a3_c24b8b70_a81a664b_a2bfe8a1,
        0x106aa070_f40e3585_d6990624_d192e819,
    ],
    [
        0x34b0bcb5_2748774c_1e376c08_19a4c116,
        0x682e6ff3_5b9cca4f_4ed8aa4a_391c0cb3,
        0x8cc70208_84c87814_78a5636f_748f82ee,
        0xc67178f2_bef9a3f7_a4506ceb_90befffa,
    ],
];

const fn round16(mut x: U256, w: &U512, i: usize) -> U256 {
    let k = &K[i];
    x = round4(x, 0, w, k);
    x = round4(x, 1, w, k);
    x = round4(x, 2, w, k);
    round4(x, 3, w, k)
}

#[inline(always)]
const fn w_round(w0: u32, w1: u32, w9: u32, we: u32) -> u32 {
    add3(SMALL1.get(we), w9, SMALL0.get(w1), w0)
}

#[inline(always)]
const fn wi(w: &U512, i: usize) -> u128 {
    w[i & 3]
}

//   0123|4567|89AB|CDEF
//   0123|0123|0123|0123
// 0:WR  |    | R  |  R
// 1: WR |    |  R |   R
// 2:R WR|    |   R|
// 3: R W|R   |    |R
const fn w_round4(w: &U512, i: usize) -> u128 {
    let w10 = wi(w, i + 1) as u32;
    let [_, w21, w22, w23] = to_u32x4(wi(w, i + 2));
    let [w30, _, w32, w33] = to_u32x4(wi(w, i + 3));
    let mut w0 = to_u32x4(w[i]);
    w0[0] = w_round(w0[0], w0[1], w21, w32);
    w0[1] = w_round(w0[1], w0[2], w22, w33);
    w0[2] = w_round(w0[2], w0[3], w23, w0[0]);
    w0[3] = w_round(w0[3], w10, w30, w0[1]);
    from_u32x4(w0)
}

const fn w_round16(mut w: U512) -> U512 {
    w[0] = w_round4(&w, 0);
    w[1] = w_round4(&w, 1);
    w[2] = w_round4(&w, 2);
    w[3] = w_round4(&w, 3);
    w
}

pub const INIT: U256 = [
    0xf70e5939_3070dd17_367cd507_c1059ed8,
    0xbefa4fa4_64f98fa7_68581511_ffc00b31,
];

pub const fn compress(mut w: U512) -> U256 {
    let mut x: U256 = INIT;
    x = round16(x, &w, 0);
    w = w_round16(w);
    x = round16(x, &w, 1);
    w = w_round16(w);
    x = round16(x, &w, 2);
    w = w_round16(w);
    x = round16(x, &w, 3);
    x = u32x8_add(&x, &INIT);
    x[1] |= 0xFFFF_FFFF << 96;
    x
}

#[cfg(test)]
mod test {
    use super::{compress, U256};

    const A: U256 = compress([0x8000_0000, 0, 0, 0]);

    #[test]
    fn test() {
        assert_eq!(
            A,
            [
                0x288234c4_476102bb_2a3a2bc9_d14a028c,
                0xFFFFFFFF_c5b3e42f_828ea62a_15a2b01f
            ]
        );
    }

    #[test]
    fn runtime_test() {
        assert_eq!(
            compress([0x8000_0000, 0, 0, 0]),
            [
                0x288234c4_476102bb_2a3a2bc9_d14a028c,
                0xFFFFFFFF_c5b3e42f_828ea62a_15a2b01f
            ]
        );
    }
}
