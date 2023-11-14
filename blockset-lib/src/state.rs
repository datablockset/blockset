use std::io::{self, Write};

use io_trait::Io;

pub struct State<'a, T: Io> {
    io: &'a T,
    prior: usize,
    start_time: T::Instant,
    prior_elapsed: f64,
    left: f64,
}

pub fn mb(b: u64) -> String {
    (b / 1_000_000).to_string() + " MB"
}

fn time(mut t: u64) -> String {
    let h = t / 3600;
    t %= 3600;
    let m = t / 60;
    format!("{}:{:02}:{:02}", h, m, t % 60)
}

impl<'a, T: Io> State<'a, T> {
    pub fn new(io: &'a T) -> Self {
        Self {
            io,
            prior: 0,
            start_time: io.now(),
            prior_elapsed: -1.0,
            left: f64::MAX,
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
        if p == 0.0 {
            return Ok(());
        }
        let percent = (p * 100.0) as u8;
        let current = self.io.now();
        let elapsed = (current - self.start_time.clone()).as_secs_f64();
        println!("elapsed: {}, {}", elapsed, elapsed - self.prior_elapsed);
        if elapsed - self.prior_elapsed < 0.01 {
            return Ok(());
        }
        println!("!!!");
        self.prior_elapsed = elapsed;
        let new_left = elapsed * (1.0 - p) / p;
        if new_left < self.left {
            self.left = new_left;
        }
        let r =
            s.to_owned() + &percent.to_string() + "%, time left: " + &time(self.left as u64) + ".";
        self.set(&r)
    }
}
impl<'a, T: Io> Drop for State<'a, T> {
    fn drop(&mut self) {
        let _ = self.set("");
    }
}
