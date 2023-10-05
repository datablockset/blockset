#[derive(Default)]
pub struct BitVec32 {
    pub v: u64,
    pub len: u8,
}

impl BitVec32 {
    pub const fn new(v: u32, len: u8) -> Self {
        Self { v: v as u64, len }
    }
    pub fn push(&mut self, f: &mut impl FnMut(u32), size: u8, b: BitVec32) {
        assert!(size <= 32);
        self.v |= b.v << self.len;
        self.len += b.len;
        let mask = (1 << size) - 1;
        loop {
            if self.len < size {
                return;
            }
            f((self.v & mask) as u32);
            self.len -= size;
            self.v >>= size;
        }
    }
}
