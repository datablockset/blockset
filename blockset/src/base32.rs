pub const fn to_char(v: u8) -> u8 {
    b"0123456789abcdefghjkmnpqrstvwxyz"[v as usize & 0x1F]
}

pub const fn to_byte(x: u8) -> Option<u8> {
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
    use super::{to_byte, to_char};

    #[test]
    fn test() {
        for i in 0..32 {
            assert_eq!(to_byte(to_char(i)).unwrap(), i);
        }
        for i in 0..32 {
            assert_eq!(to_byte(to_char(i).to_ascii_lowercase()).unwrap(), i);
        }
        assert_eq!(to_byte(b'i').unwrap(), 1);
        assert_eq!(to_byte(b'I').unwrap(), 1);
        assert_eq!(to_byte(b'l').unwrap(), 1);
        assert_eq!(to_byte(b'L').unwrap(), 1);
        assert_eq!(to_byte(b'o').unwrap(), 0);
        assert_eq!(to_byte(b'O').unwrap(), 0);
    }

    #[test]
    fn fail_test() {
        assert_eq!(to_byte(b'$'), None);
        assert_eq!(to_byte(b'U'), None);
        assert_eq!(to_byte(b'u'), None);
    }
}