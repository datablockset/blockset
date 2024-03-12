use std::io::{self, Read, Seek};

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub total: u64,
    pub current: u64,
}

pub trait Progress {
    fn position(&mut self) -> io::Result<u64>;
}

impl<T: Read + Seek> Progress for T {
    fn position(&mut self) -> io::Result<u64> {
        self.stream_position()
    }
}
