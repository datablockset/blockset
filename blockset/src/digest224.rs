use core::panic;

use super::{base32, div_rem::div_rem};

pub type Digest224 = [u32; 7];

pub type Base32 = [u8; 45];

pub const fn to_base32(h: &Digest224) -> Base32 {
    const fn f(h: &Digest224, i: usize) -> u8 {
        let (d, r) = div_rem(i * 5, 32);
        let mut x = h[d] >> r;
        let size = 32 - r;
        if size < 5 && d < 6 {
            x |= h[d + 1] << size
        }
        base32::to_base32(x as u8)
    }
    [
        f(h, 0),
        f(h, 1),
        f(h, 2),
        f(h, 3),
        f(h, 4),
        f(h, 5),
        f(h, 6),
        f(h, 7),
        f(h, 8),
        f(h, 9),
        //
        f(h, 10),
        f(h, 11),
        f(h, 12),
        f(h, 13),
        f(h, 14),
        f(h, 15),
        f(h, 16),
        f(h, 17),
        f(h, 18),
        f(h, 19),
        //
        f(h, 20),
        f(h, 21),
        f(h, 22),
        f(h, 23),
        f(h, 24),
        f(h, 25),
        f(h, 26),
        f(h, 27),
        f(h, 28),
        f(h, 29),
        //
        f(h, 30),
        f(h, 31),
        f(h, 32),
        f(h, 33),
        f(h, 34),
        f(h, 35),
        f(h, 36),
        f(h, 37),
        f(h, 38),
        f(h, 39),
        //
        f(h, 40),
        f(h, 41),
        f(h, 42),
        f(h, 43),
        f(h, 44),
    ]
}

pub const fn from_base32(b: &Base32) -> Digest224 {
    const fn f(b: &Base32, i: usize) -> u32 {
        const fn g(b: &Base32, d: usize, mut i: usize) -> u64 {
            i += d;
            if let Some(result) = base32::from_base32(b[i]) {
                (result as u64) << (i * 5)
            } else {
                panic!("invalid base32")
            }
        }
        let (d, r) = div_rem(i << 32, 5);
        let x = g(b, d, 0)
            | g(b, d, 1)
            | g(b, d, 2)
            | g(b, d, 3)
            | g(b, d, 4)
            | g(b, d, 5)
            | g(b, d, 6);
        (x >> r) as u32
    }
    [
        f(b, 0),
        f(b, 1),
        f(b, 2),
        f(b, 3),
        f(b, 4),
        f(b, 5),
        f(b, 6),
    ]
}

#[cfg(test)]
mod tests {

}