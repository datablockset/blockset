use crate::bit_vec32::BitVec32;

pub const fn to_base32(v: u8) -> char {
    b"0123456789abcdefghjkmnpqrstvwxyz"[v as usize & 0x1F] as char
}

const fn to_ascii(x: char) -> Option<u8> {
    if x <= '\u{7F}' {
        Some(x as u8)
    } else {
        None
    }
}

pub const fn from_base32(x: char) -> Option<u8> {
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

pub fn encode(i: &mut impl Iterator<Item = BitVec32>) -> (String, BitVec32) {
    let mut result = String::default();
    let mut a = BitVec32::default();
    for b in i {
        a.push(&mut |v| result.push(to_base32(v as u8)), 5, b)
    }
    (result, a)
}

pub fn decode(i: &mut impl Iterator<Item = char>) -> Option<(Vec<u32>, BitVec32)> {
    let mut result = Vec::default();
    let mut a = BitVec32::default();
    for b in i {
        if let Some(v5) = from_base32(b) {
            a.push(&mut |v| result.push(v), 32, BitVec32::new(v5 as u32, 5));
        } else {
            return None;
        }
    }
    Some((result, a))
}

#[cfg(test)]
mod test {
    use super::{from_base32, to_base32};

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

    #[test]
    fn fail_test() {
        assert_eq!(from_base32('$'), None);
        assert_eq!(from_base32('U'), None);
        assert_eq!(from_base32('u'), None);
    }

    #[test]
    fn unicode_test() {
        assert_eq!(from_base32('🦀'), None);
        assert_eq!(from_base32('🐙'), None);
        assert_eq!(from_base32('\u{0130}'), None);
        assert_eq!(from_base32('\u{80}'), None);
        assert_eq!(from_base32('\u{31}'), Some(1));
    }
}