use std::io::{self, Write};

pub struct State<'a, T: Write> {
    stdout: &'a mut T,
    prior: usize,
}

impl<'a, T: Write> State<'a, T> {
    pub fn new(stdout: &'a mut T) -> Self {
        Self { stdout, prior: 0 }
    }
    pub fn set(&mut self, s: &str) -> io::Result<()> {
        let mut vec = Vec::default();
        vec.resize(self.prior, 8);
        vec.extend_from_slice(s.as_bytes());
        self.stdout.write_all(&vec)?;
        self.prior = s.len();
        Ok(())
    }
    pub fn set_progress(&mut self, b: u64, p: u8) -> io::Result<()> {
        let s = (b / 1_000_000).to_string() + " MB, " + &p.to_string() + "%.";
        self.set(&s)
    }
}
impl<'a, T: Write> Drop for State<'a, T> {
    fn drop(&mut self) {
        let _ = self.set("");
    }
}
