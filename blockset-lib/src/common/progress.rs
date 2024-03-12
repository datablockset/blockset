use std::io::{self, Read, Seek, SeekFrom};

pub struct State {
    pub total: u64,
    pub current: u64,
}

pub trait Progress {
    fn len(&mut self) -> io::Result<u64>;
    fn position(&mut self) -> io::Result<u64>;
    fn progress(&mut self) -> io::Result<State> {
        let current = self.position()?;
        let total = self.len()?;
        Ok(State { total, current })
    }
}

impl<T: Read + Seek> Progress for T {
    fn len(&mut self) -> io::Result<u64> {
        let current = self.stream_position()?;
        let total = self.seek(SeekFrom::End(0))?;
        self.seek(SeekFrom::Start(current))?;
        Ok(total)
    }
    fn position(&mut self) -> io::Result<u64> {
        self.stream_position()
    }
}
