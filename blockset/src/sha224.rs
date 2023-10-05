use crate::{digest224::Digest224, overflow32::{add4, add, add3}, sigma32::{BIG1, BIG0, SMALL1, SMALL0}, u32x8::{to_u32x8, to_u256, u32x8_add, U256}, sha224x::{INIT, self}, u32x16::{to_u32x16, U512, to_u512}, u32x4::get_u32};

type Buffer512 = [u32; 16];

const fn round(
    x: &U256,
    i: usize,
    w: &u128,
    k: &u128,
) -> U256 {
    let [a, b, c, d, e, f, g, h] = to_u32x8(x);
    let t1 = add4(h, BIG1.get(e), (e & f) ^ (!e & g), get_u32(*k, i), get_u32(*w, i));
    let t2 = add(BIG0.get(a), (a & b) ^ (a & c) ^ (b & c));
    to_u256(&[add(t1, t2), a, b, c, add(d, t1), e, f, g])
}

const K: [Buffer512; 4] = [
    to_u32x16(&sha224x::K[0]),
    to_u32x16(&sha224x::K[1]),
    to_u32x16(&sha224x::K[2]),
    to_u32x16(&sha224x::K[3]),
];

const fn round4(mut x: U256, w: &U512, k: &U512, i: usize) -> U256 {
    let w = w[i];
    let k = k[i];
    x = round(&x, 0, &w, &k);
    x = round(&x, 1, &w, &k);
    x = round(&x, 2, &w, &k);
    round(&x, 3, &w, &k)
}

const fn round16(mut x: U256, w: &Buffer512, j: usize) -> U256 {
    let k = &sha224x::K[j];
    let w = to_u512(w);
    x = round4(x, &w, k, 0);
    x = round4(x, &w, k, 1);
    x = round4(x, &w, k, 2);
    round4(x, &w, k, 3)
}

#[inline(always)]
const fn w_get(w: &Buffer512, i: usize) -> u32 {
    w[i & 0xF]
}

#[inline(always)]
const fn wi(w: &Buffer512, i: usize) -> u32 {
    add3(
        SMALL1.get(w_get(w, i + 0xE)),
        w_get(w, i + 9),
        SMALL0.get(w_get(w, i + 1)),
        w[i],
    )
}

const fn next_w(mut w: Buffer512) -> Buffer512 {
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

pub const fn compress(mut w: Buffer512) -> Digest224 {
    let mut x: U256 =INIT;
    x = round16(x, &w, 0);
    w = next_w(w);
    x = round16(x, &w, 1);
    w = next_w(w);
    x = round16(x, &w, 2);
    w = next_w(w);
    x = round16(x, &w, 3);
    let x = to_u32x8(&u32x8_add(&x, &INIT));
    [
        x[0], x[1], x[2], x[3], x[4], x[5], x[6],
    ]
}

#[cfg(test)]
mod tests {
    use crate::{static_assert::static_assert, u32x16::to_u32x16};

    use super::{compress, Digest224};

    pub const fn eq(
        [a0, a1, a2, a3, a4, a5, a6]: Digest224,
        [b0, b1, b2, b3, b4, b5, b6]: Digest224,
    ) -> bool {
        a0 == b0 && a1 == b1 && a2 == b2 && a3 == b3 && a4 == b4 && a5 == b5 && a6 == b6
    }

    const A: Digest224 = compress(to_u32x16(&[0x8000_0000, 0, 0, 0]));

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
