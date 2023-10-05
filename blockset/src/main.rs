use blockset::{run, Io};
use std::{
    env::{args, Args},
    fs::File,
    io,
    process::ExitCode,
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

    fn open(&mut self, path: &str) -> io::Result<Self::File> {
        File::open(path)
    }
}

fn main() -> Result<(), String> {
    run(&mut RealIo::default()).map_err(str::to_string)
}
