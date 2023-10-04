pub const fn to_base32(v: u8) -> u8 {
    b"0123456789abcdefghjkmnpqrstvwxyz"[v as usize & 0x1F]
}

pub const fn from_base32(x: u8) -> Option<u8> {
    let v = x.to_ascii_lowercase();
    Some(match v {
        b'0'..=b'9' => v - b'0',
        b'a'..=b'h' => v - (b'a' - 10),
        b'i' => 1,
        b'j'..=b'k' => v - (b'a' - 9),
        b'l' => 1,
        b'm'..=b'n' => v - (b'a' - 8),
        b'o' => 0,
        b'p'..=b't' => v - (b'a' - 7),
        // U skipped
        b'v'..=b'z' => v - (b'a' - 6),
        _ => return None,
    })
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
        assert_eq!(from_base32(b'i').unwrap(), 1);
        assert_eq!(from_base32(b'I').unwrap(), 1);
        assert_eq!(from_base32(b'l').unwrap(), 1);
        assert_eq!(from_base32(b'L').unwrap(), 1);
        assert_eq!(from_base32(b'o').unwrap(), 0);
        assert_eq!(from_base32(b'O').unwrap(), 0);
    }

    #[test]
    fn fail_test() {
        assert_eq!(from_base32(b'$'), None);
        assert_eq!(from_base32(b'U'), None);
        assert_eq!(from_base32(b'u'), None);
    }
}
