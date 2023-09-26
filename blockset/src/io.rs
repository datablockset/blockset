use std::io::{self, Read};

trait Io {
    type File: Read;
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
    };

    use super::Io;

    struct MockIo {
        file_map: HashMap<String, Vec<u8>>,
    }

    impl Io for MockIo {
        type File = Cursor<Vec<u8>>;

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
            file_map: HashMap::default(),
        };
        io.file_map
            .insert("test.txt".to_string(), "Hello, world!".as_bytes().to_vec());
        let result = io.read_to_string("test.txt").unwrap();
        assert_eq!(result, "Hello, world!");
    }
}
