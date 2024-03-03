use std::io::{Write, self};

pub trait Print: Write {
    fn print(&mut self, s: &str) -> io::Result<()> {
        self.write_all(s.as_bytes())
    }
    fn println(&mut self, s: &str) -> io::Result<()> {
        self.print(s)?;
        self.print("\n")
    }
}

impl<T: Write> Print for T {}