use crate::digest224::Digest224;

struct BigSigma(u32, u32, u32);

impl BigSigma {
    #[inline(always)]
    const fn get(&self, v: u32) -> u32 {
        v.rotate_right(self.0) ^ v.rotate_right(self.1) ^ v.rotate_right(self.2)
    }
}

struct SmallSigma(u32, u32, u8);

impl SmallSigma {
    #[inline(always)]
    const fn get(&self, v: u32) -> u32 {
        v.rotate_right(self.0) ^ v.rotate_right(self.1) ^ (v >> self.2)
    }
}

#[inline(always)]
const fn add(a: u32, b: u32) -> u32 {
    a.overflowing_add(b).0
}

#[inline(always)]
const fn add2(a: u32, b: u32, c: u32) -> u32 {
    add(add(a, b), c)
}

#[inline(always)]
const fn add3(a: u32, b: u32, c: u32, d: u32) -> u32 {
    add(add(a, b), add(c, d))
}

#[inline(always)]
const fn add4(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    add2(add2(a, b, c), d, e)
}

type Digest256 = [u32; 8];

const BIG_S0: BigSigma = BigSigma(2, 13, 22);
const BIG_S1: BigSigma = BigSigma(6, 11, 25);
const SMALL_S0: SmallSigma = SmallSigma(7, 18, 3);
const SMALL_S1: SmallSigma = SmallSigma(17, 19, 10);

type Buffer = [u32; 16];

const fn round([a, b, c, d, e, f, g, h]: Digest256, i: usize, w: &Buffer, k: &Buffer) -> Digest256 {
    let t1 = add4(h, BIG_S1.get(e), (e & f) ^ (!e & g), k[i], w[i]);
    let t2 = add(BIG_S0.get(a), (a & b) ^ (a & c) ^ (b & c));
    [add(t1, t2), a, b, c, add(d, t1), e, f, g]
}

const K: [Buffer; 4] = [
    [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, //
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, //
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, //
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, //
    ],
    [
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, //
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, //
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, //
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, //
    ],
    [
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, //
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, //
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, //
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, //
    ],
    [
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, //
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, //
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, //
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2, //
    ],
];

const fn round16(mut x: Digest256, w: &Buffer, j: usize) -> Digest256 {
    let k = &K[j];
    x = round(x, 0, w, k);
    x = round(x, 1, w, k);
    x = round(x, 2, w, k);
    x = round(x, 3, w, k);
    x = round(x, 4, w, k);
    x = round(x, 5, w, k);
    x = round(x, 6, w, k);
    x = round(x, 7, w, k);
    x = round(x, 8, w, k);
    x = round(x, 9, w, k);
    x = round(x, 10, w, k);
    x = round(x, 11, w, k);
    x = round(x, 12, w, k);
    x = round(x, 13, w, k);
    x = round(x, 14, w, k);
    round(x, 15, w, k)
}

#[inline(always)]
const fn w_get(w: &Buffer, i: usize) -> u32 {
    w[i & 0xF]
}

#[inline(always)]
const fn wi(w: &Buffer, i: usize) -> u32 {
    add3(
        SMALL_S1.get(w_get(w, i + 0xE)),
        w_get(w, i + 9),
        SMALL_S0.get(w_get(w, i + 1)),
        w[i],
    )
}

const fn next_w(mut w: Buffer) -> Buffer {
    w[0x0] = wi(&w, 0x0);
    w[0x1] = wi(&w, 0x1);
    w[0x2] = wi(&w, 0x2);
    w[0x3] = wi(&w, 0x3);
    w[0x4] = wi(&w, 0x4);
    w[0x5] = wi(&w, 0x5);
    w[0x6] = wi(&w, 0x6);
    w[0x7] = wi(&w, 0x7);
    w[0x8] = wi(&w, 0x8);
    w[0x9] = wi(&w, 0x9);
    w[0xA] = wi(&w, 0xA);
    w[0xB] = wi(&w, 0xB);
    w[0xC] = wi(&w, 0xC);
    w[0xD] = wi(&w, 0xD);
    w[0xE] = wi(&w, 0xE);
    w[0xF] = wi(&w, 0xF);
    w
}

const SHA224_INIT: Digest256 = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];

pub const fn compress(mut w: Buffer) -> Digest224 {
    let mut x: Digest256 = SHA224_INIT;
    x = round16(x, &w, 0);
    w = next_w(w);
    x = round16(x, &w, 1);
    w = next_w(w);
    x = round16(x, &w, 2);
    w = next_w(w);
    x = round16(x, &w, 3);
    [
        add(x[0], SHA224_INIT[0]),
        add(x[1], SHA224_INIT[1]),
        add(x[2], SHA224_INIT[2]),
        add(x[3], SHA224_INIT[3]),
        add(x[4], SHA224_INIT[4]),
        add(x[5], SHA224_INIT[5]),
        add(x[6], SHA224_INIT[6]),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{static_assert::static_assert, digest224::eq};

    use super::{compress, Digest224};

    const A: Digest224 = compress([0x8000_0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    const _: () = static_assert(eq(
        compress([0x8000_0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        [
            0xd14a028c, 0x2a3a2bc9, 0x476102bb, 0x288234c4, 0x15a2b01f, 0x828ea62a, 0xc5b3e42f,
        ],
    ));

    #[test]
    fn test() {
        assert!(eq(
            A,
            [0xd14a028c, 0x2a3a2bc9, 0x476102bb, 0x288234c4, 0x15a2b01f, 0x828ea62a, 0xc5b3e42f]
        ));
        assert_eq!(
            A,
            [0xd14a028c, 0x2a3a2bc9, 0x476102bb, 0x288234c4, 0x15a2b01f, 0x828ea62a, 0xc5b3e42f]
        );
    }

    #[test]
    fn runtime_test() {
        assert_eq!(
            compress([0x8000_0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            [0xd14a028c, 0x2a3a2bc9, 0x476102bb, 0x288234c4, 0x15a2b01f, 0x828ea62a, 0xc5b3e42f]
        );
    }
}
