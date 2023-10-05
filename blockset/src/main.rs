use std::{process::ExitCode, env::{Args, args}, fs::File, io};
use blockset::{Io, run};

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

    fn open(&mut self, path: &str) -> io::Result<Self::File> {
        File::open(path)
    }
}

fn main() -> ExitCode {
    run(&mut RealIo::default())
}