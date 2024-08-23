use core::iter::once;

use nanvm_lib::common::{cast::Cast, default::default};

type OctetString = Vec<u8>;

#[derive(PartialEq, Debug)]
struct ObjectIdentifier {
    a0: u8,
    a1: u8,
    a2: Vec<u128>,
}

#[derive(PartialEq, Debug)]
struct BitString {
    padding: u8,
    value: Vec<u8>,
}

#[derive(PartialEq, Debug)]
struct Sequence(Vec<Any>);

#[derive(PartialEq, Debug)]
enum Any {
    Bool(bool),
    Integer(i128),
    OctetString(Vec<u8>),
    ObjectIdentifier(ObjectIdentifier),
    BitString(BitString),
    Sequence(Sequence),
}

fn d<T: Serialize>(a: &mut dyn Iterator<Item = u8>) -> T {
    let len = if let Some(len) = a.next() {
        if len < 0x80 {
            len as usize
        } else {
            read_u128(&mut a.take(len as usize & 0x78)).0 as usize
        }
    } else {
        0
    };
    T::deserialize(&mut a.take(len))
}

impl Any {
    fn serialize(self) -> Vec<u8> {
        fn f<T: Serialize>(v: T) -> Vec<u8> {
            let mut v = v.serialize();
            let len = v.len() as i128;
            let mut result = [T::TAG].cast();
            if len < 0x80 {
                result.push(len as u8);
            } else {
                let mut int = write_i128(len, false);
                result.push(int.len() as u8 | 0x80);
                result.append(&mut int);
            }
            result.append(&mut v);
            result
        }
        match self {
            Any::Bool(v) => f(v),
            Any::Integer(v) => f(v),
            Any::OctetString(v) => f(v),
            Any::BitString(v) => f(v),
            Any::ObjectIdentifier(v) => f(v),
            Any::Sequence(v) => f(v),
        }
    }
    fn deserialize(a: &mut dyn Iterator<Item = u8>) -> Option<Any> {
        if let Some(tag) = a.next() {
            match tag {
                bool::TAG => Some(Any::Bool(d(a))),
                i128::TAG => Some(Any::Integer(d(a))),
                OctetString::TAG => Some(Any::OctetString(d(a))),
                BitString::TAG => Some(Any::BitString(d(a))),
                ObjectIdentifier::TAG => Some(Any::ObjectIdentifier(d(a))),
                Sequence::TAG => Some(Any::Sequence(d(a))),
                _ => None,
            }
        } else {
            None
        }
    }
}

trait Serialize: Sized {
    const TAG: u8;
    fn serialize(self) -> Vec<u8>;
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Self;
}

impl Serialize for bool {
    const TAG: u8 = 1;
    fn serialize(self) -> Vec<u8> {
        [if self { 0xFF } else { 0 }].cast()
    }
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Self {
        if let Some(v) = a.next() {
            v != 0
        } else {
            false
        }
    }
}

fn write_i128(n: i128, signed: bool) -> Vec<u8> {
    let leading = if n.is_negative() {
        n.leading_ones()
    } else {
        n.leading_zeros()
    } - signed as u32;
    let len = 16 - (leading >> 3);
    let mut result = Vec::with_capacity(len as usize);
    let mut i = len - 1;
    loop {
        result.push((n >> (i << 3)) as u8);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    result
}

fn read_u128(i: &mut impl Iterator<Item = u8>) -> (u128, i32) {
    let mut result = 0;
    let mut bits = 0;
    for v in i {
        result = (result << 8) | (v as u128);
        bits += 8;
    }
    (result, bits)
}

impl Serialize for i128 {
    const TAG: u8 = 2;
    fn serialize(self) -> Vec<u8> {
        write_i128(self, true)
    }
    fn deserialize(i: &mut impl Iterator<Item = u8>) -> Self {
        let (mut result, c) = read_u128(i);
        if result >> (c - 1) == 1 && c < 128 {
            result |= u128::MAX << c
        }
        result as i128
    }
}

impl Serialize for BitString {
    const TAG: u8 = 3;
    fn serialize(self) -> Vec<u8> {
        once(self.padding).chain(self.value).collect()
    }
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Self {
        let padding = a.next().unwrap_or_default();
        Self {
            padding,
            value: a.collect(),
        }
    }
}

impl Serialize for OctetString {
    const TAG: u8 = 4;
    fn serialize(self) -> Vec<u8> {
        self
    }
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Self {
        a.collect()
    }
}

impl Serialize for Sequence {
    const TAG: u8 = 0x30;
    fn serialize(self) -> Vec<u8> {
        let mut result = Vec::default();
        for a in self.0 {
            result.append(&mut a.serialize())
        }
        result
    }
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Self {
        let mut result: Vec<_> = default();
        while let Some(v) = Any::deserialize(a) { result.push(v) }
        Self(result)
    }
}

const OI: u8 = 40;

impl Serialize for ObjectIdentifier {
    const TAG: u8 = 6;
    fn serialize(self) -> Vec<u8> {
        let mut result = [self.a0 * OI + self.a1].cast();
        for a in self.a2 {
            let mut len = 1.max(((128 + 6) - a.leading_zeros()) / 7) - 1;
            loop {
                let f = len > 0;
                result.push(((a >> (len * 7)) as u8 & 0x7F) | ((f as u8) << 7));
                if !f {
                    break;
                }
                len -= 1;
            }
        }
        result
    }
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Self {
        let a01 = a.next().unwrap_or_default();
        let mut a2: Vec<u128> = default();
        while let Some(mut v) = a.next() {
            let mut x = 0;
            loop {
                x |= (v & 0x7F) as u128;
                if v >> 7 == 0 {
                    break;
                }
                x <<= 7;
                v = a.next().unwrap_or_default();
            }
            a2.push(x);
        }
        Self {
            a0: a01 / OI,
            a1: a01 % OI,
            a2,
        }
    }
}

#[cfg(test)]
mod test {
    use core::fmt;

    use nanvm_lib::common::{cast::Cast, default::default};
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{Any, BitString, ObjectIdentifier, Serialize};

    #[wasm_bindgen_test]
    #[test]
    fn i128_test() {
        assert_eq!((-1i128).leading_ones(), 128);
        assert_eq!(-1i128 as u128, u128::MAX);
    }

    fn f<T: Serialize + PartialEq + fmt::Debug>(v: T, a: &[u8]) {
        let i = &mut a.into_iter().map(|x| *x);
        let m = T::deserialize(i);
        assert_eq!(i.next(), None);
        assert_eq!(m, v);
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

    fn any_f(v: Any, a: &[u8]) {
        let mut i = a.into_iter().map(|x| *x);
        assert_eq!(Any::deserialize(&mut i).unwrap(), v);
        assert_eq!(i.next(), None);
        assert_eq!(v.serialize(), a);
    }

    #[wasm_bindgen_test]
    #[test]
    fn any_test() {
        any_f(Any::Bool(true), &[1, 1, 0xFF]);
        any_f(Any::Bool(false), &[1, 1, 0]);
        any_f(
            Any::Integer(i128::MIN),
            &[2, 16, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        any_f(
            Any::Integer(i128::MIN + 1),
            &[2, 16, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );
        any_f(
            Any::ObjectIdentifier(ObjectIdentifier {
                a0: 2,
                a1: 0,
                a2: [1, 235].cast(),
            }),
            &[6, 4, 80, 1, 0x81, 0x6b],
        );
        any_f(
            Any::OctetString([123, 45, 67, 89].cast()),
            &[4, 4, 123, 45, 67, 89],
        );
        any_f(
            Any::BitString(BitString {
                padding: 3,
                value: [98, 76, 54, 32, 10].cast(),
            }),
            &[3, 6, 3, 98, 76, 54, 32, 10],
        );
        //any_f(
        //    Any::Sequence([].cast()),
        //    &[0x30, 0]
        //);
    }

    #[wasm_bindgen_test]
    #[test]
    fn oi_test() {
        f(
            ObjectIdentifier {
                a0: 0,
                a1: 3,
                a2: default(),
            },
            &[3],
        );
        f(
            ObjectIdentifier {
                a0: 1,
                a1: 2,
                a2: default(),
            },
            &[42],
        );
        f(
            ObjectIdentifier {
                a0: 2,
                a1: 17,
                a2: [127, 5, 0x89].cast(),
            },
            &[97, 127, 5, 0x81, 9],
        );
        f(
            ObjectIdentifier {
                a0: 3,
                a1: 39,
                a2: [0x82, 0x4345, 0x26789A].cast(),
            },
            &[159, 0x81, 2, 0x81, 0x86, 0x45, 0x81, 0x99, 0xF1, 0x1A],
        );
    }
}
