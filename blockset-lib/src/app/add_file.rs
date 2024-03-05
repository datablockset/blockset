use std::io;

use io_trait::Io;

use crate::{cdt::tree_add::TreeAdd, common::print::Print};

use super::{invalid_input, is_to_posix_eol, read_to_tree_file};

pub fn add_file<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    a: &mut T::Args,
    storage: impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let stdout = &mut io.stdout();
    let path = a.next().ok_or(invalid_input("missing file name"))?;
    let to_posix_eol = is_to_posix_eol(a)?;
    let f = io.open(&path)?;
    let k = read_to_tree_file(to_posix_eol, storage(io), f, io, display_new)?;
    stdout.println([k.as_str()])
}
