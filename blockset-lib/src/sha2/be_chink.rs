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
}
