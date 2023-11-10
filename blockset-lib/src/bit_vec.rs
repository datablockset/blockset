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
    pub fn push(&mut self, size: u8, overflow: &mut impl FnMut(u32), b: BitVec) {
        assert!(size <= 32);
        self.value |= b.value << self.len;
        self.len += b.len;
        let mask = (1u64 << size) - 1;
        loop {
            if self.len < size {
                return;
            }
            overflow((self.value & mask) as u32);
            self.len -= size;
            self.value >>= size;
        }
    }
}
