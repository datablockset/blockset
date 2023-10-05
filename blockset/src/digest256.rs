use crate::{compress, digest224::Digest224, sha224};

pub type Digest256 = [u32; 8];

const SUFFIX: usize = 7;

const HASH_SUFFIX: u32 = 0xFFFF_FFFF;

const LEN_MAX: usize = 0xF8;

const fn len(d: &Digest256) -> usize {
    (d[7] >> 24) as usize
}

const fn hash(&[a0, a1, a2, a3, a4, a5, a6, _]: &Digest256) -> Digest224 {
    [a0, a1, a2, a3, a4, a5, a6]
}

const fn merge(a: &Digest256, b: &Digest256) -> Digest256 {
    let a_len = len(a);
    let b_len = len(b);
    let len = a_len + b_len;
    if len <= LEN_MAX {
        todo!()
    } else {
        let h = compress([
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], //
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], //
        ]);
        [h[0], h[1], h[2], h[3], h[4], h[5], h[6], HASH_SUFFIX]
    }
}

#[cfg(test)]
mod test {}
