use crate::u256::U256;

const LEN_MAX: usize = 31;

// in bytes
// - 0..31 - data
// - 32 - hash
const fn len(&[_, b]: &U256) -> usize {
    let result = (b >> 120) as usize;
    if result == 0xFF { LEN_MAX + 1 } else { result >> 3 }
}

/*
const fn merge(a: &U256, b: &U256) -> U256 {
    let a_len = len(a);
    let b_len = len(b);
    let len = a_len + b_len;
    if len <= LEN_MAX {

    } else {

    }
}
*/