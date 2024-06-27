use std::io::{self, Write};

use io_trait::Io;

use crate::uint::u64x::div_rem;

pub struct StatusLine<'a, T: Io> {
    io: &'a T,
    prior: usize,
    start_time: T::Instant,
    prior_elapsed: f64,
}

pub fn mb(b: u64) -> String {
    (b / 1_000_000).to_string() + " MB"
}

fn time(s: u64) -> String {
    let (h, s) = div_rem(s, 3600);
    let (m, s) = div_rem(s, 60);
    format!("{}:{:02}:{:02}", h, m, s)
}

impl<'a, T: Io> StatusLine<'a, T> {
    pub fn new(io: &'a T) -> Self {
        Self {
            io,
            prior: 0,
            start_time: io.now(),
            prior_elapsed: -1.0,
        }
    }
    fn set(&mut self, s: &str) -> io::Result<()> {
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
        if p == 0.0 {
            return Ok(());
        }
        let percent = (p * 100.0) as u8;
        let current = self.io.now();
        let elapsed = (current - self.start_time.clone()).as_secs_f64();
        if elapsed - self.prior_elapsed < 0.01 {
            return Ok(());
        }
        self.prior_elapsed = elapsed;
        let left = elapsed * (1.0 - p) / p;
        let r = s.to_owned() + &percent.to_string() + "%. Time left: " + &time(left as u64) + ".";
        self.set(&r)
    }
}
impl<'a, T: Io> Drop for StatusLine<'a, T> {
    fn drop(&mut self) {
        let _ = self.set("");
    }
}
