use core::{iter::empty, ops::Deref, ptr::read_unaligned};

use nanvm_lib::common::cast::Cast;

#[derive(PartialEq, Debug)]
struct OctetString(Vec<u8>);

#[derive(PartialEq, Debug)]
struct ObjectIdentifier {
    a0: u8,
    a1: u8,
    a2: Vec<i128>,
}

#[derive(PartialEq, Debug)]
struct BitString {
    padding: u8,
    value: Vec<u8>,
}

struct Sequence(Vec<Any>);

#[derive(PartialEq, Debug)]
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
        fn f<T: Serialize>(v: T) -> Vec<u8> {
            let mut v = v.serialize();
            let len = v.len() as i128;
            let mut result = [T::TAG].cast();
            if len < 0x80 {
                result.push(len as u8);
            } else {
                let mut int = write_int(len, false);
                result.push(int.len() as u8 | 0x80);
                result.append(&mut int);
            }
            result.append(&mut v);
            result
        }
        match self {
            Any::Bool(v) => f(v),
            Any::Integer(v) => f(v),
            _ => todo!(),
        }
    }
    fn deserialize(a: &mut impl Iterator<Item = u8>) -> Any {
        fn f<T: Serialize>(a: &mut impl Iterator<Item = u8>) -> T {
            let len = if let Some(len) = a.next() {
                if len < 0x80 {
                    len as usize
                } else {
                    read_uint(a).0 as usize
                }
            } else {
                0
            };
            T::deserialize(&mut a.take(len))
        }
        if let Some(tag) = a.next() {
            match tag {
                bool::TAG => Any::Bool(f(a)),
                i128::TAG => Any::Integer(f(a)),
                _ => todo!(),
            }
        } else {
            Any::Bool(false)
        }
    }
}

trait Serialize {
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

fn write_int(n: i128, signed: bool) -> Vec<u8> {
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

fn read_uint(i: &mut impl Iterator<Item = u8>) -> (u128, i32) {
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
        write_int(self, true)
    }
    fn deserialize(i: &mut impl Iterator<Item = u8>) -> Self {
        let (mut result, c) = read_uint(i);
        if result >> (c - 1) == 1 && c < 128 {
            result |= u128::MAX << c
        }
        result as i128
    }
}

#[cfg(test)]
mod test {
    use core::fmt;

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{Any, Serialize};

    #[wasm_bindgen_test]
    #[test]
    fn i128_test() {
        assert_eq!((-1i128).leading_ones(), 128);
        assert_eq!(-1i128 as u128, u128::MAX);
    }

    fn f<T: Serialize + PartialEq + fmt::Debug>(v: T, a: &[u8]) {
        let m = T::deserialize(&mut a.into_iter().map(|x| *x));
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
        assert_eq!(Any::deserialize(&mut a.into_iter().map(|x| *x)), v);
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
    }
}
