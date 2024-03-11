use std::io;

use io_trait::{Io, Metadata};

use crate::{cdt::tree_add::TreeAdd, common::print::Print};

use super::{add::Add, add_dir::add_dir, invalid_input, is_to_posix_eol, read_to_tree_file};

pub fn add_file<'a, T: Io, S: 'a + TreeAdd, F: Fn(&'a T) -> S>(
    add: &Add<'a, T, S, F>,
    path: &str,
) -> io::Result<String> {
    read_to_tree_file(
        add.to_posix_eol,
        (add.storage)(add.io),
        add.io.open(path)?,
        add.io,
        add.display_new,
    )
}

pub fn add_entry<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    a: &mut T::Args,
    storage: &'a impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing file name"))?;
    let to_posix_eol = is_to_posix_eol(a)?;
    let add = Add {
        io,
        storage,
        to_posix_eol,
        display_new,
    };
    if io.metadata(&path)?.is_dir() {
        add_dir(&add, &path)
    } else {
        let k = add_file(&add, &path)?;
        io.stdout().println([k.as_str()])
    }
}
