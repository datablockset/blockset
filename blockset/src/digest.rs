use crate::{u256::{bitor, shl, U256}, compress};

const LEN_MAX: usize = 0xF8;

const LEN_HI_POS: usize = 0x78;

// in bits
// - 0..0xF8 - data
// - 0xFF - hash
const fn len(&[_, b]: &U256) -> usize {
    (b >> LEN_HI_POS) as usize
}

const DATA_MASK: u128 = (1 << LEN_HI_POS) - 1;

const fn remove_len(&[a, b]: &U256) -> U256 {
    [a, b & DATA_MASK]
}

const fn set_len(&[a, b]: &U256, len: usize) -> U256 {
    [a, b | ((len as u128) << LEN_HI_POS)]
}

const fn merge(a: &U256, b: &U256) -> U256 {
    let a_len = len(a);
    let b_len = len(b);
    let len = a_len + b_len;
    if len <= LEN_MAX {
        set_len(&bitor(&remove_len(a), &shl(&remove_len(b), a_len)), len)
    } else {
        compress([*a, *b])
    }
}

const fn to_digest(a: u8) -> U256 {
    set_len(&[a as u128, 0], 8)
}

#[cfg(test)]
mod test {
    use crate::{u256::{U256, shl}, digest::{to_digest, len, merge, remove_len, set_len, LEN_HI_POS}};

    #[test]
    fn bit_test() {
        let r = (8 as u128) << LEN_HI_POS;
        assert_eq!(r, 0x0800_0000_0000_0000_0000_0000_0000_0000);
        let mut a = to_digest(0x12);
        let mut b = to_digest(0x34);
        let a_len = len(&a);
        assert_eq!(a_len, 8);
        a = remove_len(&a);
        assert_eq!(a, [0x12, 0]);
        b = remove_len(&b);
        assert_eq!(b, [0x34, 0]);
        b = shl(&b, a_len);
        assert_eq!(b, [0x3400, 0]);
    }

    #[test]
    fn test() {
        const A: U256 = to_digest(0x12);
        assert_eq!(A, [0x12, 0x0800_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&A), 8);
        const B: U256 = to_digest(0x34);
        assert_eq!(B, [0x34, 0x0800_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&B), 8);
        let C: U256 = merge(&A, &B);
        // println!("{:x?}", C);
        // assert_eq!(C, [0x3412, 0x1000_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&C), 16);
    }
}