#![cfg(not(tarpaulin_include))]

use std::{
    env::{args, Args},
    fs::{self, create_dir, File},
    io::{self, Stdout},
};

use crate::{io::DirEntry, Io, Metadata};

impl Metadata for fs::Metadata {
    fn len(&self) -> u64 {
        self.len()
    }
    fn is_dir(&self) -> bool {
        self.is_dir()
    }
}

impl DirEntry for fs::DirEntry {
    type Metadata = fs::Metadata;
    fn path(&self) -> String {
        self.path().to_str().unwrap().to_string()
    }
    fn metadata(&self) -> io::Result<Self::Metadata> {
        self.metadata()
    }
}

#[derive(Default)]
pub struct RealIo();

impl Io for RealIo {
    type Args = Args;

    type Stdout = Stdout;
    type File = File;
    type Metadata = fs::Metadata;
    type DirEntry = fs::DirEntry;

    fn args(&self) -> Self::Args {
        args()
    }

    fn create(&self, path: &str) -> io::Result<Self::File> {
        File::create(path)
    }

    fn open(&self, path: &str) -> io::Result<Self::File> {
        File::open(path)
    }

    fn metadata(&self, path: &str) -> io::Result<fs::Metadata> {
        fs::metadata(path)
    }

    fn read_dir(&self, path: &str) -> io::Result<Vec<Self::DirEntry>> {
        fs::read_dir(path)?.collect()
    }

    fn create_dir(&self, path: &str) -> io::Result<()> {
        create_dir(path)
    }

    fn stdout(&self) -> Self::Stdout {
        io::stdout()
    }
}
