use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Read, Write},
    iter::once,
    rc::Rc,
    vec,
};

use crate::{io::Metadata, Io};

type VecRef = Rc<RefCell<Vec<u8>>>;

pub struct VirtualIo {
    pub args: Vec<String>,
    pub file_map: HashMap<String, VecRef>,
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

#[derive(Debug)]
pub struct MemFile {
    vec_ref: VecRef,
    pos: usize,
}

impl MemFile {
    fn new(vec_ref: VecRef) -> Self {
        Self { vec_ref, pos: 0 }
    }
}

impl Read for MemFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let sorce = &self.vec_ref.borrow()[self.pos..];
        let len = sorce.len().min(buf.len());
        buf[..len].copy_from_slice(&sorce[..len]);
        self.pos += len;
        Ok(len)
    }
}

impl Write for MemFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.vec_ref.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Io for VirtualIo {
    type File = MemFile;
    type Args = vec::IntoIter<String>;
    fn args(&self) -> Self::Args {
        self.args.clone().into_iter()
    }
    fn print(&mut self, s: &str) {
        self.stdout.push_str(s);
    }
    fn metadata(&self, path: &str) -> io::Result<Metadata> {
        self.file_map
            .get(path)
            .map(|_| Metadata::default())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }
    fn create(&mut self, path: &str) -> io::Result<Self::File> {
        let vec_ref = Rc::new(RefCell::new(Vec::default()));
        self.file_map.insert(path.to_string(), vec_ref.clone());
        Ok(MemFile::new(vec_ref))
    }
    fn open(&self, path: &str) -> io::Result<Self::File> {
        self.file_map
            .get(path)
            .map(|v| MemFile::new(v.clone()))
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }
    fn create_dir(&mut self, _path: &str) -> io::Result<()> {
        Ok(())
    }
}
