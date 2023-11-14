use std::io::{self, Write};

use io_trait::Io;

pub struct State<'a, T: Io> {
    io: &'a T,
    prior: usize,
    start_time: T::Instant,
    left: u64,
}

pub fn mb(b: u64) -> String {
    (b / 1_000_000).to_string() + " MB"
}

fn time(t: u64) -> String {
    let mut result = String::default();
    let mut t = t;
    if t >= 3600 {
        result += &((t / 3600).to_string() + "h ");
        t %= 3600;
    }
    if t >= 60 {
        result += &((t / 60).to_string() + "m ");
        t %= 60;
    }
    result += &(t.to_string() + "s");
    result
}

impl<'a, T: Io> State<'a, T> {
    pub fn new(io: &'a T) -> Self {
        Self {
            io,
            prior: 0,
            start_time: io.now(),
            left: u64::MAX,
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
        let current = self.io.now();
        let elapsed = current - self.start_time.clone();
        let left = if p == 0.0 {
            "".to_string()
        } else {
            let new_left = (elapsed.as_secs_f64() * (1.0 - p) / p) as u64;
            if new_left < self.left {
                self.left = new_left;
            }
            ", time left: ".to_owned() + &time(self.left)
        } + ".";
        let r = s.to_owned() + &percent.to_string() + "%" + &left;
        self.set(&r)
    }
}
impl<'a, T: Io> Drop for State<'a, T> {
    fn drop(&mut self) {
        let _ = self.set("");
    }
}
