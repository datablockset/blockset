use std::io::{self, Read, Seek, SeekFrom};

pub struct State {
    pub total: u64,
    pub current: u64,
}

pub trait Progress {
    fn progress(&mut self) -> io::Result<State>;
}

impl<T: Read + Seek> Progress for T {
    fn progress(&mut self) -> io::Result<State> {
        let current = self.stream_position()?;
        let total = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(current))?;
        Ok(State { total, current })
    }
}
