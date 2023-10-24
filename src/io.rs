use std::{
    fmt,
    io::{self, Read, Write},
};

#[allow(clippy::len_without_is_empty)]
pub trait Metadata {
    fn len(&self) -> u64;
    fn is_dir(&self) -> bool;
}

pub trait DirEntry {
    type Metadata: Metadata;
    fn path(&self) -> String;
    fn metadata(&self) -> io::Result<Self::Metadata>;
}

pub trait Io {
    type Args: Iterator<Item = String>;
    type File: Read + Write + fmt::Debug;
    type Stdout: Write;
    type Metadata: Metadata;
    type DirEntry: DirEntry;
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
    fn read_dir(&self, path: &str) -> io::Result<Vec<Self::DirEntry>>;
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
    fn read_dir_type(&self, path: &str, is_dir: bool) -> io::Result<Vec<Self::DirEntry>> {
        let mut result = Vec::default();
        for i in self.read_dir(path)? {
            if i.metadata()?.is_dir() == is_dir {
                result.push(i);
            }
        }
        Ok(result)
    }
}

pub enum OperationResult {
    Ok(usize),
    Pending,
    Err(io::Error),
}

trait Operation {
    fn get_result(&mut self) -> OperationResult;
}

trait AsyncFile {
    type Operation<'a>: Operation
    where
        Self: 'a;
    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> io::Result<Self::Operation<'a>>;
    fn write<'a>(&'a mut self, buffer: &'a [u8]) -> io::Result<Self::Operation<'a>>;
}

trait AsyncIo {
    type File: AsyncFile;
    fn create(&self, path: &str) -> io::Result<Self::File>;
    fn open(&self, path: &str) -> io::Result<Self::File>;
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
    fn test_dir_fail() {
        let io = VirtualIo::new(&[]);
        assert!(io.write("a/test.txt", "Hello, world!".as_bytes()).is_err());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_write_recursively() {
        let io = VirtualIo::new(&[]);
        assert!(io
            .write_recursively("a/test.txt", "Hello, world!".as_bytes())
            .is_ok());
        assert!(io
            .write_recursively("a/test2.txt", "Hello, world!".as_bytes())
            .is_ok());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_dir_rec() {
        let io = VirtualIo::new(&[]);
        assert!(io
            .write_recursively("a/b/test.txt", "Hello, world!".as_bytes())
            .is_ok());
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
