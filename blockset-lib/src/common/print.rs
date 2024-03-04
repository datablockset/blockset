use core::iter::once;
use std::io::{self, Write};

pub trait Print: Write {
    fn print<'a>(&mut self, s: impl IntoIterator<Item = &'a str>) -> io::Result<()> {
        for i in s {
            self.write_all(i.as_bytes())?;
        }
        Ok(())
    }
    fn println<'a>(&mut self, s: impl IntoIterator<Item = &'a str>) -> io::Result<()> {
        self.print(s.into_iter().chain(once("\n")))
    }
}

impl<T: Write> Print for T {}

#[cfg(test)]
mod test {
    use std::io::{self, Cursor, Write};

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::Print;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let mut w = Cursor::new(Vec::new());
        w.print(["a", "b"]).unwrap();
        w.println(["c", "d"]).unwrap();
        assert_eq!(w.into_inner(), b"abcd\n");
    }

    struct X();

    impl Write for X {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "x"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_error() {
        let mut w = X();
        w.print(["a", "b"]).unwrap_err();
    }
}
