use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::{self, Read, Write},
    iter::once,
    rc::Rc,
    vec,
};

use crate::io::Io;

#[derive(Debug, Clone)]
pub struct Metadata {
    len: u64,
}

impl crate::io::Metadata for Metadata {
    fn len(&self) -> u64 {
        self.len
    }
}

#[derive(Debug, Default, Clone)]
pub struct VecRef(Rc<RefCell<Vec<u8>>>);

impl VecRef {
    pub fn to_string(&self) -> String {
        let mut result = String::default();
        for &c in self.0.borrow().iter() {
            if c == 8 {
                result.pop();
            } else {
                result.push(c as char);
            }
        }
        result
    }
}

impl Write for VecRef {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct FileSystem {
    directory_set: HashSet<String>,
    file_map: HashMap<String, VecRef>,
}

impl FileSystem {
    pub fn check_dir(&self, path: &str) -> io::Result<()> {
        if let Some(d) = path.rfind('/').map(|i| &path[..i]) {
            if !self.directory_set.contains(d) {
                return Err(not_found());
            }
        }
        Ok(())
    }
}

pub struct DirEntry {
    path: String,
    metadata: Metadata,
}

impl crate::io::DirEntry for DirEntry {
    type Metadata = Metadata;
    fn path(&self) -> String {
        self.path.clone()
    }
    fn metadata(&self) -> io::Result<Self::Metadata> {
        Ok(self.metadata.clone())
    }
}

pub struct VirtualIo {
    pub args: Vec<String>,
    pub fs: RefCell<FileSystem>,
    pub stdout: VecRef,
}

impl VirtualIo {
    pub fn new(args: &[&str]) -> Self {
        Self {
            args: once("blockset".to_string())
                .chain(args.iter().map(|v| v.to_string()))
                .collect(),
            fs: Default::default(),
            stdout: VecRef::default(),
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
        let sorce = &self.vec_ref.0.borrow()[self.pos..];
        let len = sorce.len().min(buf.len());
        buf[..len].copy_from_slice(&sorce[..len]);
        self.pos += len;
        Ok(len)
    }
}

impl Write for MemFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.vec_ref.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.vec_ref.flush()
    }
}

fn not_found() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "file not found")
}

fn check_path(a: &str) -> io::Result<()> {
    if a.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '_' || c == '.')
    {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid file name",
        ))
    }
}

impl Io for VirtualIo {
    type File = MemFile;
    type Stdout = VecRef;
    type Args = vec::IntoIter<String>;
    type Metadata = Metadata;
    type DirEntry = DirEntry;
    fn args(&self) -> Self::Args {
        self.args.clone().into_iter()
    }
    fn metadata(&self, path: &str) -> io::Result<Metadata> {
        let fs = self.fs.borrow();
        fs.file_map
            .get(path)
            .map(|v| Metadata {
                len: v.0.borrow().len() as u64,
            })
            .ok_or_else(not_found)
    }
    fn create(&self, path: &str) -> io::Result<Self::File> {
        let mut fs = self.fs.borrow_mut();
        fs.check_dir(path)?;
        let vec_ref = VecRef::default();
        check_path(path)?;
        fs.file_map.insert(path.to_string(), vec_ref.clone());
        Ok(MemFile::new(vec_ref))
    }
    fn create_dir(&self, path: &str) -> io::Result<()> {
        let mut fs = self.fs.borrow_mut();
        fs.directory_set.insert(path.to_string());
        Ok(())
    }
    fn open(&self, path: &str) -> io::Result<Self::File> {
        let fs = self.fs.borrow();
        fs.check_dir(path)?;
        check_path(path)?;
        fs.file_map
            .get(path)
            .map(|v| MemFile::new(v.clone()))
            .ok_or_else(not_found)
    }
    fn stdout(&self) -> VecRef {
        self.stdout.clone()
    }

    fn read_dir(&self, path: &str) -> io::Result<Vec<DirEntry>> {
        let fs = self.fs.borrow();
        let d = fs.directory_set.iter().map(|d| DirEntry {
            path: d.to_string(),
            metadata: Metadata { len: 0 },
        });
        let f = fs.file_map.iter().map(|(k, v)| DirEntry {
            path: k.to_owned(),
            metadata: Metadata {
                len: v.0.borrow().len() as u64,
            },
        });
        let i = d.chain(f);
        let x = i
            .filter(|p| {
                if let Some((a, _)) = p.path.rsplit_once('/') {
                    a == path
                } else {
                    false
                }
            })
            .collect();
        Ok(x)
    }
}
