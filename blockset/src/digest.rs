use crate::u256::{U256, shl, bitor};

const LEN_MAX: usize = 31;

const LEN_POS: usize = 120;

// in bytes
// - 0..31 - data
// - 32 - hash
const fn len(&[_, b]: &U256) -> usize {
    let result = (b >> LEN_POS) as usize;
    if result == 0xFF {
        LEN_MAX + 1
    } else {
        result >> 3
    }
}

const DATA_MASK: u128 = (1 << 120) - 1;

const fn remove_len(&[a, b]: &U256) -> U256 {
    [a, b & DATA_MASK]
}

const fn set_len(&[a, b]: &U256, len: usize) -> U256 {
    [a, b | ((len as u128) << LEN_POS)]
}

const fn merge(a: &U256, b: &U256) -> U256 {
    let a_len = len(a);
    let b_len = len(b);
    let len = a_len + b_len;
    if len <= LEN_MAX {
        set_len(&bitor(&remove_len(a), &shl(&remove_len(b), a_len)), len)
    } else {
        todo!()
    }
}
