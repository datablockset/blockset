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
        assert!(S <= 32);
        self.value |= b.value << self.len;
        self.len += b.len;
        let mask = (1u64 << S) - 1;
        loop {
            if self.len < S {
                return;
            }
            overflow((self.value & mask) as u32);
            self.len -= S;
            self.value >>= S;
        }
    }
}

struct BitVecSplit<const S: u8, I: Iterator<Item = BitVec>> {
    iter: I,
    remainder: BitVec,
}

impl<const S: u8, I: Iterator<Item = BitVec>> BitVecSplit<S, I> {
    const MASK: u32 = (1u32 << S) - 1;
}

impl<const S: u8, I: Iterator<Item = BitVec>> Iterator for BitVecSplit<S, I> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut value = self.remainder.value as u64;
        while self.remainder.len < S {
            if let Some(b) = self.iter.next() {
                value |= b.value << self.remainder.len;
                self.remainder.len += b.len;
                continue;
            }
            if self.remainder.len == 0 {
                return None;
            }
            break;
        }
        self.remainder.value = value >> S;
        Some((value as u32) & Self::MASK)
    }
}