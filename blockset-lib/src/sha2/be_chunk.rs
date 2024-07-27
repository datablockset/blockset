use crate::uint::u512x::{self, U512};

#[derive(Default)]
pub struct BeChunk {
    pub data: U512,
    pub len: u16,
}

impl BeChunk {
    pub const fn new(data: U512, len: u16) -> Self {
        Self { data, len }
    }
    pub const fn default() -> Self {
        Self::new(u512x::_0, 0)
    }
    pub const fn chain(mut self, BeChunk { data, len }: BeChunk) -> (Option<U512>, Self) {
        let d = self.len as i32;
        self.data = u512x::bitor(&self.data, &u512x::shl(&data, -d));
        self.len += len;
        let r0 = if self.len >= 0x200 {
            let r = self.data;
            self.len -= 0x200;
            self.data = u512x::shl(&data, 0x200 - d);
            Some(r)
        } else {
            None
        };
        (r0, self)
    }
    pub const fn u8(v: u8) -> Self {
        BeChunk::new(u512x::be((v as u128) << 0x78, 0, 0, 0), 8)
    }
}
