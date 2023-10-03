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

pub type Digest224 = [u32; 7];

pub type Digest256 = [u32; 8];

const BIG_S0: BigSigma = BigSigma(2, 13, 22);
const BIG_S1: BigSigma = BigSigma(6, 11, 25);
const SMALL_S0: SmallSigma = SmallSigma(7, 18, 3);
const SMALL_S1: SmallSigma = SmallSigma(17, 19, 10);

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

const INIT: Digest256 = [
    0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
];

type Buffer = [u32; 16];

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

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const E: usize = 4;
const F: usize = 5;
const G: usize = 6;
const H: usize = 7;

const fn round(
    [a, b, c, d, e, f, g, mut h]: Digest256,
    i: usize,
    w: &Buffer,
    k: &Buffer,
) -> Digest256 {
    let t1 = add4(h, BIG_S1.get(e), (e & f) ^ (!e & g), k[i], w[i]);
    let t2 = add(BIG_S0.get(a), (a & b) ^ (a & c) ^ (b & c));
    [add(t1, t2), a, b, c, add(d, t1), e, f, g]
}

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

const fn wi(a: u32, b: u32, c: u32, d: u32) -> u32 {
    add3(SMALL_S1.get(a), b, SMALL_S0.get(c), d)
}

const fn next_w(
    [mut w0, mut w1, mut w2, mut w3, mut w4, mut w5, mut w6, mut w7, mut w8, mut w9, mut wa, mut wb, mut wc, mut wd, mut we, mut wf]: Buffer,
) -> Buffer {
    w0 = wi(we, w9, w1, w0);
    w1 = wi(wf, wa, w2, w1);
    w2 = wi(w0, wb, w3, w2);
    w3 = wi(w1, wc, w4, w3);
    w4 = wi(w2, wd, w5, w4);
    w5 = wi(w3, we, w6, w5);
    w6 = wi(w4, wf, w7, w6);
    w7 = wi(w5, w0, w8, w7);
    w8 = wi(w6, w1, w9, w8);
    w9 = wi(w7, w2, wa, w9);
    wa = wi(w8, w3, wb, wa);
    wb = wi(w9, w4, wc, wb);
    wc = wi(wa, w5, wd, wc);
    wd = wi(wb, w6, we, wd);
    we = wi(wc, w7, wf, we);
    wf = wi(wd, w8, w0, wf);
    [
        w0, w1, w2, w3, w4, w5, w6, w7, w8, w9, wa, wb, wc, wd, we, wf,
    ]
}

const fn compress(mut w: Buffer) -> Digest224 {
    let mut x: Digest256 = INIT;
    x = round16(x, &w, 0);
    w = next_w(w);
    x = round16(x, &w, 1);
    w = next_w(w);
    x = round16(x, &w, 2);
    w = next_w(w);
    x = round16(x, &w, 3);
    [
        add(x[A], INIT[A]),
        add(x[B], INIT[B]),
        add(x[C], INIT[C]),
        add(x[D], INIT[D]),
        add(x[E], INIT[E]),
        add(x[F], INIT[F]),
        add(x[G], INIT[G]),
    ]
}

#[cfg(test)]
mod tests {
    use super::{compress, Digest224};

    const A: Digest224 = compress([0x8000_0000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    #[test]
    fn test() {
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
