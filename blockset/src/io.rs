use std::io::{self, Read};

pub trait Io {
    type Args: Iterator<Item = String>;
    type File: Read;
    fn args(&self) -> Self::Args;
    fn print(&mut self, s: &str);
    fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }
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
    use std::{
        collections::HashMap,
        io::{self, Cursor},
        vec,
    };

    use super::Io;

    struct MockIo {
        args: Vec<String>,
        file_map: HashMap<String, Vec<u8>>,
        stdout: String,
    }

    impl Io for MockIo {
        type File = Cursor<Vec<u8>>;
        type Args = vec::IntoIter<String>;
        fn args(&self) -> Self::Args {
            self.args.clone().into_iter()
        }
        fn print(&mut self, s: &str) {
            self.stdout.push_str(s);
        }
        fn open(&mut self, path: &str) -> io::Result<Self::File> {
            self.file_map
                .get(path)
                .map(|data| Cursor::new(data.clone()))
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
        }
    }

    #[test]
    fn test() {
        let mut io = MockIo {
            args: Vec::default(),
            file_map: HashMap::default(),
            stdout: String::default(),
        };
        io.file_map
            .insert("test.txt".to_string(), "Hello, world!".as_bytes().to_vec());
        let result = io.read_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, world!");
    }
}
