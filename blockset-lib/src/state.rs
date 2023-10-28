use std::io::{self, Write};

pub struct State<'a, T: Write> {
    stdout: &'a mut T,
    prior: usize,
}

pub fn mb(b: u64) -> String {
    (b / 1_000_000).to_string() + " MB"
}

pub fn progress(b: u64, p: u8) -> String {
    mb(b) + ", " + &p.to_string() + "%."
}

impl<'a, T: Write> State<'a, T> {
    pub fn new(stdout: &'a mut T) -> Self {
        Self { stdout, prior: 0 }
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
        self.stdout.write_all(&vec)?;
        self.prior = s.len();
        Ok(())
    }
}
impl<'a, T: Write> Drop for State<'a, T> {
    fn drop(&mut self) {
        let _ = self.set("");
    }
}
