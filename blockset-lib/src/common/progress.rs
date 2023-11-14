use std::io;

use io_trait::{File, Metadata};

pub struct State {
    pub total: u64,
    pub current: u64,
}

pub trait Progress {
    fn progress(&mut self) -> io::Result<State>;
}

impl<T: File> Progress for T {
    fn progress(&mut self) -> io::Result<State> {
        Ok(State {
            total: self.metadata()?.len(),
            current: self.stream_position()?,
        })
    }
}
