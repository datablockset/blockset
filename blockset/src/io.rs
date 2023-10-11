use std::io::{self, Read, Write};

pub trait Io {
    type Args: Iterator<Item = String>;
    type File: Read + Write;
    fn args(&self) -> Self::Args;
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }
    fn create(&mut self, path: &str) -> io::Result<Self::File>;
    fn open(&mut self, path: &str) -> io::Result<Self::File>;
    fn read_to_string(&mut self, path: &str) -> io::Result<String> {
        let mut file = self.open(path)?;
        let mut result = String::default();
        file.read_to_string(&mut result)?;
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::virtual_io::VirtualIo;

    use super::Io;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let mut io = VirtualIo::new(&[]);
        io.create("test.txt")
            .unwrap()
            .write_all("Hello, world!".as_bytes())
            .unwrap();
        let result = io.read_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, world!");
    }
}
