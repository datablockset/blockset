use std::{collections::HashMap, io::{Cursor, self}, vec, iter::once};

use crate::Io;

pub struct VirtualIo {
    pub args: Vec<String>,
    pub file_map: HashMap<String, Vec<u8>>,
    pub stdout: String,
}

impl VirtualIo {
    pub fn new(args: &[&str]) -> Self {
        Self {
            args: once("blockset".to_string())
                .chain(args.iter().map(|v| v.to_string()))
                .collect(),
            file_map: HashMap::default(),
            stdout: String::default(),
        }
    }
}

impl Io for VirtualIo {
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