use blockset::{run, Io, Metadata};
use std::{
    env::{args, Args},
    fs::{create_dir, File},
    io,
};

#[derive(Default)]
struct RealIo();

impl Io for RealIo {
    type Args = Args;

    type File = File;

    fn args(&self) -> Self::Args {
        args()
    }

    fn print(&mut self, text: &str) {
        print!("{}", text);
    }

    fn create(&mut self, path: &str) -> io::Result<Self::File> {
        File::create(path)
    }

    fn open(&self, path: &str) -> io::Result<Self::File> {
        File::open(path)
    }

    fn metadata(&self, path: &str) -> io::Result<Metadata> {
        std::fs::metadata(path).map(|_| Metadata::default())
    }

    fn create_dir(&mut self, path: &str) -> io::Result<()> {
        create_dir(path)
    }
}

fn main() -> Result<(), String> {
    run(&mut RealIo::default())
}
