#![cfg(not(tarpaulin_include))]

use std::{
    env::{args, Args},
    fs::{self, create_dir, File},
    io::{self, Stdout},
};

use io_trait::Io;

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
