use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Read, Write},
    iter::once,
    rc::Rc,
    vec,
};

use crate::io::Io;

#[derive(Debug, Clone)]
pub struct Metadata {
    len: u64,
    is_dir: bool,
}

impl crate::io::Metadata for Metadata {
    fn len(&self) -> u64 {
        self.len
    }
    fn is_dir(&self) -> bool {
        self.is_dir
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
enum Entity {
    #[default]
    Dir,
    File(VecRef),
}

impl Entity {
    fn metadata(&self) -> Metadata {
        match self {
            Entity::Dir => Metadata {
                len: 0,
                is_dir: true,
            },
            Entity::File(x) => Metadata {
                len: x.0.borrow().len() as u64,
                is_dir: false,
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct FileSystem {
    entity_map: HashMap<String, Entity>,
}

impl FileSystem {
    pub fn check_dir(&self, path: &str) -> io::Result<()> {
        if let Some(Entity::Dir) = self.entity_map.get(path) {
            Ok(())
        } else {
            Err(not_found())
        }
    }
    pub fn check_parent(&self, path: &str) -> io::Result<()> {
        if let Some(d) = path.rfind('/').map(|i| &path[..i]) {
            self.check_dir(d)
        } else {
            Ok(())
        }
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
        fs.entity_map
            .get(path)
            .map(Entity::metadata)
            .ok_or_else(not_found)
    }
    fn create(&self, path: &str) -> io::Result<Self::File> {
        let mut fs = self.fs.borrow_mut();
        fs.check_parent(path)?;
        let vec_ref = VecRef::default();
        check_path(path)?;
        fs.entity_map
            .insert(path.to_string(), Entity::File(vec_ref.clone()));
        Ok(MemFile::new(vec_ref))
    }
    fn create_dir(&self, path: &str) -> io::Result<()> {
        let mut fs = self.fs.borrow_mut();
        fs.entity_map.insert(path.to_string(), Entity::Dir);
        Ok(())
    }
    fn open(&self, path: &str) -> io::Result<Self::File> {
        let fs = self.fs.borrow();
        fs.check_parent(path)?;
        check_path(path)?;
        fs.entity_map
            .get(path)
            .map(|v| {
                if let Entity::File(x) = v {
                    Some(MemFile::new(x.to_owned()))
                } else {
                    None
                }
            })
            .flatten()
            .ok_or_else(not_found)
    }
    fn stdout(&self) -> VecRef {
        self.stdout.clone()
    }

    fn read_dir(&self, path: &str) -> io::Result<Vec<DirEntry>> {
        let fs = self.fs.borrow();
        fs.check_dir(path)?;
        let i = fs.entity_map.iter().map(|(p, e)| DirEntry {
            path: p.to_owned(),
            metadata: e.metadata(),
        });
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
