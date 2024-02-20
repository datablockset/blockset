use std::io::{self, Read};

use super::progress::{self, Progress};

trait ReadEx: Read {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let mut buf = [0];
        Ok(if self.read(&mut buf)? == 1 {
            Some(buf[0])
        } else {
            None
        })
    }
}

impl<R: Read> ReadEx for R {}

pub struct ToPosixEol<R: Read> {
    read: R,
    last: Option<u8>,
}

impl<R: Read> ToPosixEol<R> {
    pub fn new(read: R) -> Self {
        Self { read, last: None }
    }
    fn get_one(&mut self) -> io::Result<Option<u8>> {
        self.last.take().map_or_else(|| self.read.read_byte(), |x| Ok(Some(x)))
    }
    fn next(&mut self) -> io::Result<Option<u8>> {
        // read the last item
        let mut last = if let Some(last) = self.get_one()? { last } else { return Ok(None) };
        //
        if last == b'\r' {
            if let Some(next) = self.read.read_byte()? {
                if next == b'\n' {
                    last = b'\n';
                } else {
                    self.last = Some(next);
                }
            }
        }
        Ok(Some(last))
    }
}

impl<R: Read> Read for ToPosixEol<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut i = 0;
        while i < buf.len() {
            if let Some(next) = self.next()? {
                buf[i] = next;
                i += 1;
            } else {
                break;
            }
        }
        Ok(i)
    }
}

impl<R: Read + Progress> Progress for ToPosixEol<R> {
    fn progress(&mut self) -> io::Result<progress::State> {
        self.read.progress()
    }
}

#[cfg(test)]
mod test {
    use std::io::{Cursor, Read};

    use nanvm_lib::common::default::default;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::ToPosixEol;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let cursor = Cursor::new(b"abc\r\ndef\r\n\r\nghi\r\n\re");
        let mut x = ToPosixEol::new(cursor);
        let mut b = default();
        x.read_to_end(&mut b).unwrap();
        assert_eq!(b, b"abc\ndef\n\nghi\n\re");
    }
    #[test]
    #[wasm_bindgen_test]
    fn test_overflow() {
        let c = b"\r\r";
        let cursor = Cursor::new(c);
        let mut x = ToPosixEol::new(cursor);
        let mut b = default();
        x.read_to_end(&mut b).unwrap();
        assert_eq!(b, c);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_error() {
        struct ReadError();
        impl Read for ReadError {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "read error"))
            }
        }
        let mut x = ToPosixEol::new(ReadError());
        let mut b = default();
        x.read_to_end(&mut b).unwrap_err();
    }
}
