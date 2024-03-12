use std::io;

use io_trait::Io;

use crate::{
    cdt::tree_add::TreeAdd,
    common::{print::Print, progress::State, status_line::StatusLine},
};

use super::{
    add::{posix_path, Add},
    invalid_input, is_to_posix_eol,
};

fn add_file_or_dir<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    storage: &'a impl Fn(&'a T) -> S,
    to_posix_eol: bool,
    display_new: bool,
    path: String,
) -> io::Result<String> {
    let mut add = Add {
        io,
        storage,
        to_posix_eol,
        display_new,
        status: StatusLine::new(io),
        p: State {
            total: 0,
            current: 0,
        },
    };
    add.add_file_or_dir(&path, add.io.metadata(&path)?)
}

pub fn add_entry<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    a: &mut T::Args,
    storage: &'a impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let mut path = posix_path(&a.next().ok_or(invalid_input("missing file name"))?);
    if path.ends_with('/') {
        path.pop();
    }
    let to_posix_eol = is_to_posix_eol(a)?;
    let k = add_file_or_dir(io, storage, to_posix_eol, display_new, path)?;
    io.stdout().println([k.as_str()])
}
