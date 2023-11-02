use std::io::{self, Read};

trait ReadEx: Read {
    fn read_byte(&mut self) -> io::Result<Option<u8>> {
        let mut buf = [0];
        match self.read(&mut buf)? {
            0 => Ok(None),
            1 => Ok(Some(buf[0])),
            _ => unreachable!(),
        }
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
    fn next(&mut self) -> io::Result<Option<u8>> {
        // read the last item
        let mut last = if let Some(last) = self.last.take() {
            last
        } else if let Some(last) = self.read.read_byte()? {
            last
        } else {
            return Ok(None);
        };
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

#[cfg(test)]
mod test {
    use std::io::{Cursor, Read};

    use super::ToPosixEol;

    #[test]
    fn test() {
        let cursor = Cursor::new(b"abc\r\ndef\r\n\r\nghi\r\n\re");
        let mut x = ToPosixEol::new(cursor);
        let mut b = Default::default();
        x.read_to_end(&mut b).unwrap();
        assert_eq!(b, b"abc\ndef\n\nghi\n\re");
    }
    #[test]
    fn test_overflow() {
        let c = b"\r\r";
        let cursor = Cursor::new(c);
        let mut x = ToPosixEol::new(cursor);
        let mut b = Default::default();
        x.read_to_end(&mut b).unwrap();
        assert_eq!(b, c);
    }
}
