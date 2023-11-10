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
