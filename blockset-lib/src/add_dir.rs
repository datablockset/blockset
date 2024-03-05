use std::io;

use io_trait::Io;

use crate::{app::invalid_input, common::print::Print};

pub fn add_dir<T: Io>(io: &T, mut a: T::Args) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing directory name"))?;
    io.stdout().println(["add-dir: ", path.as_str()])
}
