use std::{
    fmt,
    io::{self, Read, Write},
};

#[allow(clippy::len_without_is_empty)]
pub trait Metadata {
    fn len(&self) -> u64;
}

pub trait Io {
    type Args: Iterator<Item = String>;
    type File: Read + Write + fmt::Debug;
    type Stdout: Write;
    type Metadata: Metadata;
    fn args(&self) -> Self::Args;
    fn stdout(&self) -> Self::Stdout;
    fn metadata(&self, path: &str) -> io::Result<Self::Metadata>;
    fn create_dir(&self, path: &str) -> io::Result<()>;
    fn create(&self, path: &str) -> io::Result<Self::File>;
    fn open(&self, path: &str) -> io::Result<Self::File>;
    fn read(&self, path: &str) -> io::Result<Vec<u8>> {
        let mut file = self.open(path)?;
        let mut result = Vec::default();
        file.read_to_end(&mut result)?;
        Ok(result)
    }
    fn read_to_string(&self, path: &str) -> io::Result<String> {
        let mut file = self.open(path)?;
        let mut result = String::default();
        file.read_to_string(&mut result)?;
        Ok(result)
    }
    fn write(&self, path: &str, data: &[u8]) -> io::Result<()> {
        let mut file = self.create(path)?;
        file.write_all(data)?;
        Ok(())
    }
    fn create_dir_recursively(&self, path: &str) -> io::Result<()> {
        let mut x = String::default();
        let mut e = Ok(());
        for i in path.split('/') {
            x += i;
            e = self.create_dir(&x);
            x += "/";
        }
        e
    }
    fn write_recursively(&self, path: &str, data: &[u8]) -> io::Result<()> {
        let e = self.write(path, data);
        if let Err(er) = e {
            return if let Some((p, _)) = path.rsplit_once('/') {
                self.create_dir_recursively(p)?;
                self.write(path, data)
            } else {
                Err(er)
            };
        }
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
        let io = VirtualIo::new(&[]);
        io.write("test.txt", "Hello, world!".as_bytes()).unwrap();
        let result = io.read_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_err() {
        let io = VirtualIo::new(&[]);
        assert!(io
            .write_recursively("?", "Hello, world!".as_bytes())
            .is_err());
    }
}
