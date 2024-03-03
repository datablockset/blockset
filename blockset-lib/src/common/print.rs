use core::iter::once;
use std::io::{self, Write};

pub trait Print: Write {
    fn print<'a>(&mut self, s: impl IntoIterator<Item = &'a str>) -> io::Result<()> {
        for s in s {
            self.write_all(s.as_bytes())?;
        }
        Ok(())
    }
    fn println<'a>(&mut self, s: impl IntoIterator<Item = &'a str>) -> io::Result<()> {
        self.print(s.into_iter().chain(once("\n")))
    }
}

impl<T: Write> Print for T {}
