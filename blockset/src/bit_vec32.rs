#[derive(Default)]
pub struct BitVec32 {
    pub v: u32,
    pub len: u8,
}

impl BitVec32 {
    pub const fn new(v: u32, len: u8) -> Self {
        Self { v, len }
    }
    pub fn push(&mut self, f: &mut impl FnMut(u32) -> (), size: u8, b: BitVec32) {
        let mut v = ((b.v as u64) << self.len) | (self.v as u64);
        let mut len = self.len + b.len;
        loop {
            if len < size {
                self.v = v as u32;
                self.len = len as u8;
                return;
            }
            f(v as u32);
            len -= size;
            v >>= size;
        }
    }
}
