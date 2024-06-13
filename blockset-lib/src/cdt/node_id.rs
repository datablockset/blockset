use crate::{
    sha2::{compress::compress, sha224::SHA224},
    uint::{
        u128x::to_u32x4,
        u224x::U224,
        u256x::{bitor, shl, U256},
    },
};

const LEN_MAX: usize = 0xF8;

const LEN_HI_POS: usize = 0x78;

// in bits
// - 0..0xF8 - data
// - 0xFF - hash
pub const fn len(&[_, hi]: &U256) -> usize {
    (hi >> LEN_HI_POS) as usize
}

const DATA_MASK: u128 = (1 << LEN_HI_POS) - 1;

const fn remove_len(&[lo, hi]: &U256) -> U256 {
    [lo, hi & DATA_MASK]
}

const fn set_len(&[lo, hi]: &U256, len: usize) -> U256 {
    [lo, hi | ((len as u128) << LEN_HI_POS)]
}

pub const fn merge(a: &U256, b: &U256) -> U256 {
    let a_len = len(a);
    if a_len == 0 {
        return *b;
    }
    let b_len = len(b);
    if b_len == 0 {
        return *a;
    }
    let len = a_len + b_len;
    if len <= LEN_MAX {
        set_len(&bitor(&remove_len(a), &shl(&remove_len(b), a_len)), len)
    } else {
        let mut x = compress(SHA224, [*a, *b]);
        x[1] |= 0xFFFF_FFFF << 96;
        x
    }
}

pub const fn to_node_id(a: u8) -> U256 {
    set_len(&[a as u128, 0], 8)
}

pub const fn root(hash: &U256) -> U224 {
    let [a0, a1] = compress(SHA224, [*hash, [0, 0]]);
    let [a10, a11, a12, _] = to_u32x4(a1);
    let [a00, a01, a02, a03] = to_u32x4(a0);
    [a00, a01, a02, a03, a10, a11, a12]
}

#[cfg(test)]
mod test {
    use nanvm_lib::common::default::default;
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        cdt::node_id::{len, merge, remove_len, to_node_id, LEN_HI_POS},
        uint::u256x::{shl, U256},
    };

    #[wasm_bindgen_test]
    #[test]
    fn bit_test() {
        let r = (8 as u128) << LEN_HI_POS;
        assert_eq!(r, 0x0800_0000_0000_0000_0000_0000_0000_0000);
        let mut a = to_node_id(0x12);
        let mut b = to_node_id(0x34);
        let a_len = len(&a);
        assert_eq!(a_len, 8);
        a = remove_len(&a);
        assert_eq!(a, [0x12, 0]);
        b = remove_len(&b);
        assert_eq!(b, [0x34, 0]);
        b = shl(&b, a_len);
        assert_eq!(b, [0x3400, 0]);
    }

    #[wasm_bindgen_test]
    #[test]
    fn merge_empty_test() {
        assert_eq!(merge(&to_node_id(0x12), &default()), to_node_id(0x12));
        assert_eq!(merge(&default(), &to_node_id(0x34)), to_node_id(0x34));
    }

    #[wasm_bindgen_test]
    #[test]
    fn const_test() {
        const A: U256 = to_node_id(0x12);
        assert_eq!(A, [0x12, 0x0800_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&A), 8);
        const B: U256 = to_node_id(0x34);
        assert_eq!(B, [0x34, 0x0800_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&B), 8);
        const C: U256 = merge(&A, &B);
        assert_eq!(C, [0x3412, 0x1000_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&C), 16);
        const C2: U256 = merge(&C, &C);
        assert_eq!(C2, [0x3412_3412, 0x2000_0000_0000_0000_0000_0000_0000_0000]);
        assert_eq!(len(&C2), 0x20);
        const C4: U256 = merge(&C2, &C2);
        assert_eq!(
            C4,
            [
                0x3412_3412_3412_3412,
                0x4000_0000_0000_0000_0000_0000_0000_0000
            ]
        );
        assert_eq!(len(&C4), 0x40);
        const C8: U256 = merge(&C4, &C4);
        assert_eq!(
            C8,
            [
                0x3412_3412_3412_3412_3412_3412_3412_3412,
                0x8000_0000_0000_0000_0000_0000_0000_0000
            ]
        );
        assert_eq!(len(&C8), 0x80);
        const C16: U256 = merge(&C8, &C8);
        assert_eq!(len(&C16), 0xFF);
        const C12: U256 = merge(&C8, &C4);
        assert_eq!(
            C12,
            [
                0x3412_3412_3412_3412_3412_3412_3412_3412,
                0xC000_0000_0000_0000_3412_3412_3412_3412
            ]
        );
        const C14: U256 = merge(&C12, &C2);
        assert_eq!(
            C14,
            [
                0x3412_3412_3412_3412_3412_3412_3412_3412,
                0xE000_0000_3412_3412_3412_3412_3412_3412
            ]
        );
        const C15: U256 = merge(&C14, &C);
        assert_eq!(
            C15,
            [
                0x3412_3412_3412_3412_3412_3412_3412_3412,
                0xF000_3412_3412_3412_3412_3412_3412_3412
            ]
        );
        const C151: U256 = merge(&C15, &to_node_id(0x56));
        assert_eq!(
            C151,
            [
                0x3412_3412_3412_3412_3412_3412_3412_3412,
                0xF856_3412_3412_3412_3412_3412_3412_3412
            ]
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn runtime_test() {
        let check_len = |a, l| {
            assert_eq!(len(&a), l);
            a
        };
        let check = |a, l, e: U256| {
            check_len(a, l);
            assert_eq!(a, e);
            a
        };
        let a = check(
            to_node_id(0xEF),
            8,
            [0xEF, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let b = check(
            to_node_id(0xCD),
            8,
            [0xCD, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let ab = check(
            merge(&a, &b),
            16,
            [0xCDEF, 0x1000_0000_0000_0000_0000_0000_0000_0000],
        );
        let c = check(
            to_node_id(0xAB),
            8,
            [0xAB, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let abc = check(
            merge(&ab, &c),
            24,
            [0xAB_CDEF, 0x1800_0000_0000_0000_0000_0000_0000_0000],
        );
        let d = check(
            to_node_id(0x89),
            8,
            [0x89, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let e = check(
            to_node_id(0x67),
            8,
            [0x67, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let de = check(
            merge(&d, &e),
            16,
            [0x6789, 0x1000_0000_0000_0000_0000_0000_0000_0000],
        );
        let abcde = check(
            merge(&abc, &de),
            40,
            [0x67_89AB_CDEF, 0x2800_0000_0000_0000_0000_0000_0000_0000],
        );
        let f = check(
            to_node_id(0x45),
            8,
            [0x45, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let g = check(
            to_node_id(0x23),
            8,
            [0x23, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let h = check(
            to_node_id(0x01),
            8,
            [0x01, 0x0800_0000_0000_0000_0000_0000_0000_0000],
        );
        let gh = check(
            merge(&g, &h),
            16,
            [0x0123, 0x1000_0000_0000_0000_0000_0000_0000_0000],
        );
        let fgh = check(
            merge(&f, &gh),
            24,
            [0x01_2345, 0x1800_0000_0000_0000_0000_0000_0000_0000],
        );
        let abcdefgh = check(
            merge(&abcde, &fgh),
            64,
            [
                0x0123_4567_89AB_CDEF,
                0x4000_0000_0000_0000_0000_0000_0000_0000,
            ],
        );
        let abcdefghabcde = check(
            merge(&abcdefgh, &abcde),
            104,
            [
                0x67_89AB_CDEF_0123_4567_89AB_CDEF,
                0x6800_0000_0000_0000_0000_0000_0000_0000,
            ],
        );
        let abcdefghabcdea = check(
            merge(&abcdefghabcde, &a),
            112,
            [
                0xEF67_89AB_CDEF_0123_4567_89AB_CDEF,
                0x7000_0000_0000_0000_0000_0000_0000_0000,
            ],
        );
        let abcdefghabcdea2 = check(
            merge(&abcdefghabcdea, &abcdefghabcdea),
            224,
            [
                0xCDEF_EF67_89AB_CDEF_0123_4567_89AB_CDEF,
                0xE000_0000_EF67_89AB_CDEF_0123_4567_89AB,
            ],
        );
        check_len(merge(&abcdefghabcdea2, &abcdefghabcdea2), 255);
    }
}
