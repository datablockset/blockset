use crate::common::{ascii::to_ascii, bit_vec::BitVec};

const fn to_base32(v: u8) -> char {
    //0               1
    //0123456789ABCDEF0123456789ABCDEF
    b"0123456789abcdefghjkmnpqrstvwxyz"[v as usize & 0x1F] as char
}

const fn from_base32(x: char) -> Option<u8> {
    if let Some(mut b) = to_ascii(x) {
        b = b.to_ascii_lowercase();
        Some(match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'h' => b - (b'a' - 10),
            b'i' => 1,
            b'j'..=b'k' => b - (b'a' - 9),
            b'l' => 1,
            b'm'..=b'n' => b - (b'a' - 8),
            b'o' => 0,
            b'p'..=b't' => b - (b'a' - 7),
            // U skipped
            b'v'..=b'z' => b - (b'a' - 6),
            _ => return None,
        })
    } else {
        None
    }
}

pub trait ToBase32 {
    fn to_base32(self) -> String;
}

pub trait BitsToBase32 {
    fn bits_to_base32(self) -> (String, BitVec);
}

impl<T: Iterator<Item = BitVec>> BitsToBase32 for T {
    fn bits_to_base32(self) -> (String, BitVec) {
        let mut result = String::default();
        let mut a = BitVec::default();
        for b in self {
            a.push::<5>(&mut |v| result.push(to_base32(v as u8)), b)
        }
        (result, a)
    }
}

pub trait FromBase32: Sized {
    fn from_base32(i: &str) -> Option<Self>;
}

pub trait StrEx {
    #[allow(clippy::wrong_self_convention)]
    fn from_base32<T: FromBase32>(&self) -> Option<T>;
}

impl StrEx for str {
    fn from_base32<T: FromBase32>(&self) -> Option<T> {
        T::from_base32(self)
    }
}

impl FromBase32 for (Vec<u32>, BitVec) {
    fn from_base32(i: &str) -> Option<Self> {
        let mut result = Vec::default();
        let mut a = BitVec::default();
        for b in i.chars() {
            let v5 = from_base32(b)?;
            a.push::<32>(&mut |v| result.push(v), BitVec::new(v5 as u32, 5));
        }
        Some((result, a))
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{from_base32, to_base32};

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        for i in 0..32 {
            assert_eq!(from_base32(to_base32(i)).unwrap(), i);
        }
        for i in 0..32 {
            assert_eq!(from_base32(to_base32(i).to_ascii_lowercase()).unwrap(), i);
        }
        assert_eq!(from_base32('i').unwrap(), 1);
        assert_eq!(from_base32('I').unwrap(), 1);
        assert_eq!(from_base32('l').unwrap(), 1);
        assert_eq!(from_base32('L').unwrap(), 1);
        assert_eq!(from_base32('o').unwrap(), 0);
        assert_eq!(from_base32('O').unwrap(), 0);
    }

    #[wasm_bindgen_test]
    #[test]
    fn fail_test() {
        assert_eq!(from_base32('$'), None);
        assert_eq!(from_base32('U'), None);
        assert_eq!(from_base32('u'), None);
    }

    #[wasm_bindgen_test]
    #[test]
    fn unicode_test() {
        assert_eq!(from_base32('🦀'), None);
        assert_eq!(from_base32('🐙'), None);
        assert_eq!(from_base32('\u{0130}'), None);
        assert_eq!(from_base32('\u{80}'), None);
        assert_eq!(from_base32('\u{31}'), Some(1));
    }
}
