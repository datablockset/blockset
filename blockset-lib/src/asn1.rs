use core::ops::Deref;

use nanvm_lib::common::cast::Cast;

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
    fn deserialize(a: &[u8]) -> Any {
        if a.len() == 0 {
            return Any::Bool(false)
        }
        match a[0] {
            1 => {
                /*
                let a = &a[1..];
                let len = a[0] as usize;
                let a = &a[1..];
                let (len, a) = if len < 0x80 {
                    (len, a)
                } else {
                    i128::deserialize(&a[..len]) as usize
                };
                */
                Any::Bool(bool::deserialize(a.into_iter().map(|x| *x)))
            }
            _ => todo!()
        }
    }
}

trait Serialize {
    const TAG: u8;
    fn serialize(self) -> Vec<u8>;
    fn deserialize(a: impl IntoIterator<Item = u8>) -> Self;
}

impl Serialize for bool {
    const TAG: u8 = 1;
    fn serialize(self) -> Vec<u8> {
        [if self { 0xFF } else { 0 }].cast()
    }
    fn deserialize(a: impl IntoIterator<Item = u8>) -> Self {
        if let Some(v) = a.into_iter().next() {
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
        if i == 0 { break }
        i -= 1;
    }
    result
}

impl Serialize for i128 {
    const TAG: u8 = 2;
    fn serialize(self) -> Vec<u8> {
        write_int(self, true)
    }
    fn deserialize(a: impl IntoIterator<Item = u8>) -> Self {
        let mut i = a.into_iter();
        if let Some(first) = i.next() {
            let mut v = first as u128;
            let mut result = 0u128.wrapping_sub(v >> 7);
            loop {
                result = (result << 8) | v;
                if let Some(iv) = i.next() {
                    v = iv as u128;
                } else {
                    break
                }
            }
            result as i128
        } else {
            0
        }
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
        assert_eq!(T::deserialize(a.into_iter().map(|x| *x)), v);
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

    #[wasm_bindgen_test]
    #[test]
    fn any_test() {
        assert_eq!(Any::Bool(true).serialize(), [1, 1, 0xFF]);
        assert_eq!(Any::Bool(false).serialize(), [1, 1, 0]);
        assert_eq!(
            Any::Integer(i128::MIN).serialize(),
            [2, 16, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
