#[derive(Default)]
pub struct BitVec {
    pub value: u64,
    pub len: u8,
}

impl BitVec {
    pub const fn new(v: u32, len: u8) -> Self {
        Self {
            value: v as u64,
            len,
        }
    }
    pub fn push<const S: u8>(&mut self, overflow: &mut impl FnMut(u32), b: BitVec) {
        self.value |= b.value << self.len;
        self.len += b.len;
        let mask = (1u64 << S) - 1;
        while self.len >= S {
            overflow((self.value & mask) as u32);
            self.len -= S;
            self.value >>= S;
        }
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::BitVec;

    fn check(v: u32, len: u8, f: fn(v: u32, len: u8) -> BitVec) {
        let x = f(v, len);
        assert_eq!(x.value, v as u64);
        assert_eq!(x.len, len);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        check(0b1010, 4, BitVec::new);
        let x = BitVec::new(0b1010, 4);
        assert_eq!(x.value, 0b1010);
        assert_eq!(x.len, 4);
    }
}
