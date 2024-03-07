use std::io;

use io_trait::{Io, Metadata};

use crate::{cdt::tree_add::TreeAdd, common::print::Print};

use super::{add_dir::add_dir, invalid_input, is_to_posix_eol, read_to_tree_file};

pub fn add_file<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    path: &str,
    to_posix_eol: bool,
    storage: impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let f = io.open(path)?;
    let k = read_to_tree_file(to_posix_eol, storage(io), f, io, display_new)?;
    io.stdout().println([k.as_str()])
}

pub fn add_entry<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    a: &mut T::Args,
    storage: impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing file name"))?;
    let to_posix_eol = is_to_posix_eol(a)?;
    if io.metadata(&path)?.is_dir() {
        add_dir(io, &path)
    } else {
        add_file(io, &path, to_posix_eol, storage, display_new)
    }
}
