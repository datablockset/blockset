use std::io::Read;

use nanvm_lib::{big_numbers::big_int::BigInt, common::cast::Cast};

struct OctetString(Vec<u8>);

struct ObjectIdentifier {
    a0: u8,
    a1: u8,
    a2: Vec<i128>,
}

struct BitString {
    padding: u8,
    value: Vec<u8>,
}

struct Sequence(Vec<Any>);

enum Any {
    Bool(bool),
    Integer(i128),
    OctetString(OctetString),
    ObjectIdentifier(ObjectIdentifier),
    BitString(BitString),
    Sequence(Vec<Any>),
}

impl Any {
    fn serialize(self) -> Vec<u8> {
        match self {
            Any::Bool(v) => {
                let v = v.serialize();
                [bool::TAG, v.len() as u8].into_iter().chain(v).collect()
            }
            _ => todo!(),
        }
    }
}

trait Serialize {
    const TAG: u8;
    fn serialize(self) -> Vec<u8>;
    fn deserialize(a: &[u8]) -> Self;
}

impl Serialize for bool {
    const TAG: u8 = 1;
    fn serialize(self) -> Vec<u8> {
        [if self { 0xFF } else { 0 }].cast()
    }
    fn deserialize(a: &[u8]) -> Self {
        a.len() > 0 && a[0] != 0
    }
}

impl Serialize for i128 {
    const TAG: u8 = 2;
    fn serialize(self) -> Vec<u8> {
        let neg = self.is_negative();
        let u = self as u128;
        let led = if neg {
            u.leading_ones()
        } else {
            u.leading_zeros()
        };
        // 1 => 16, ... 7 => 16, 8 => 16,
        // 9 => 15,
        // ...
        // 113 => 2, ... 120 => 2
        // 121 => 1, ... 128 => 1
        let len = (16 - ((led - 1) >> 3)) as usize;
        let mut result = Vec::with_capacity(len);
        let max = len - 1;
        for i in 0..len {
            result.push((u >> ((max - i) << 3)) as u8);
        }
        result
    }
    fn deserialize(a: &[u8]) -> Self {
        if a.len() == 0 {
            return 0;
        }
        let mut result = 0u128.wrapping_sub((a[0] >> 7) as u128);
        for &i in a {
            result = (result << 8) | (i as u128);
        }
        result as i128
    }
}

#[cfg(test)]
mod test {
    use core::fmt;

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::Serialize;

    #[wasm_bindgen_test]
    #[test]
    fn i128_test() {
        assert_eq!((-1i128).leading_ones(), 128);
        assert_eq!(-1i128 as u128, u128::MAX);
    }

    fn f<T: Serialize + PartialEq + fmt::Debug>(v: T, a: &[u8]) {
        assert_eq!(T::deserialize(a), v);
        assert_eq!(v.serialize(), a);
    }

    #[wasm_bindgen_test]
    #[test]
    fn bool_test() {
        f(true, &[0xFF]);
        f(false, &[0]);
    }

    #[wasm_bindgen_test]
    #[test]
    fn integer_test() {
        // len: 1
        f(0, &[0]);
        f(1, &[1]);
        f(-1, &[0xFF]);
        f(0x7F, &[0x7F]);
        f(-0x80, &[0x80]);
        // len: 2
        f(0x80, &[0, 0x80]);
        f(-0x81, &[0xFF, 0x7F]);
        f(0x7FFF, &[0x7F, 0xFF]);
        f(-0x8000, &[0x80, 0x00]);
        // len: 16
        f(
            i128::MAX,
            &[
                0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ],
        );
        f(
            i128::MIN,
            &[0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
    }
}
