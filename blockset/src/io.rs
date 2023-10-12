use std::io::{self, Read, Write};

#[derive(Default)]
pub struct Metadata();

pub trait Io {
    type Args: Iterator<Item = String>;
    type File: Read + Write;
    fn args(&self) -> Self::Args;
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }
    fn metadata(&self, path: &str) -> io::Result<Metadata>;
    fn create(&mut self, path: &str) -> io::Result<Self::File>;
    fn open(&self, path: &str) -> io::Result<Self::File>;
    fn read(&self, path: &str) -> io::Result<Vec<u8>> {
        let mut file = self.open(path)?;
        let mut result = Vec::default();
        file.read_to_end(&mut result)?;
        Ok(result)
    }
    fn read_to_string(&mut self, path: &str) -> io::Result<String> {
        let mut file = self.open(path)?;
        let mut result = String::default();
        file.read_to_string(&mut result)?;
        Ok(result)
    }
    fn write(&mut self, path: &str, data: &[u8]) -> io::Result<()> {
        let mut file = self.create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::virtual_io::VirtualIo;

    use super::Io;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let mut io = VirtualIo::new(&[]);
        io.write("test.txt", "Hello, world!".as_bytes()).unwrap();
        let result = io.read_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, world!");
    }
}
