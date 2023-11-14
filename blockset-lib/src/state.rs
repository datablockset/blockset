use std::io::{self, Write};

use io_trait::Io;

pub struct State<'a, T: Io> {
    io: &'a T,
    prior: usize,
    start_time: T::Instant,
}

pub fn mb(b: u64) -> String {
    (b / 1_000_000).to_string() + " MB"
}

impl<'a, T: Io> State<'a, T> {
    pub fn new(io: &'a T) -> Self {
        Self {
            io,
            prior: 0,
            start_time: io.now(),
        }
    }
    pub fn set(&mut self, s: &str) -> io::Result<()> {
        let mut vec = Vec::default();
        vec.resize(self.prior, 8);
        vec.extend_from_slice(s.as_bytes());
        if s.len() < self.prior {
            let len = self.prior - s.len();
            vec.resize(self.prior * 2, 0x20);
            vec.resize(vec.len() + len, 8);
        }
        self.io.stdout().write_all(&vec)?;
        self.prior = s.len();
        Ok(())
    }
    pub fn set_progress(&mut self, s: &str, p: f64) -> io::Result<()> {
        let percent = (p * 100.0) as u8;
        let r = s.to_owned() + &percent.to_string() + "%.";
        self.set(&r)
    }
}
impl<'a, T: Io> Drop for State<'a, T> {
    fn drop(&mut self) {
        let _ = self.set("");
    }
}
