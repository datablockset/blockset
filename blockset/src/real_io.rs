#![cfg(not(tarpaulin_include))]

use std::{
    env::{args, Args},
    fs::{self, create_dir, File},
    io::{self, Stdout},
};

use crate::{Io, Metadata};

impl Metadata for fs::Metadata {
    fn len(&self) -> u64 {
        self.len()
    }
}

#[derive(Default)]
pub struct RealIo();

impl Io for RealIo {
    type Args = Args;

    type Stdout = Stdout;
    type File = File;
    type Metadata = fs::Metadata;

    fn args(&self) -> Self::Args {
        args()
    }

    /*
    fn print(&mut self, text: &str) {
        print!("{}", text);
    }
    */

    fn create(&mut self, path: &str) -> io::Result<Self::File> {
        File::create(path)
    }

    fn open(&self, path: &str) -> io::Result<Self::File> {
        File::open(path)
    }

    fn metadata(&self, path: &str) -> io::Result<fs::Metadata> {
        fs::metadata(path)
    }

    fn create_dir(&mut self, path: &str) -> io::Result<()> {
        create_dir(path)
    }

    fn stdout(&self) -> Self::Stdout {
        io::stdout()
    }
}