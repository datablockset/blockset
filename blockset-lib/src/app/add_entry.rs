use std::io;

use io_trait::{Io, Metadata};

use crate::{
    cdt::tree_add::TreeAdd,
    common::{print::Print, status_line::StatusLine},
};

use super::{add::Add, invalid_input, is_to_posix_eol};

pub fn add_entry<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    a: &mut T::Args,
    storage: &'a impl Fn(&'a T) -> S,
    display_new: bool,
) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing file name"))?;
    let to_posix_eol = is_to_posix_eol(a)?;
    let k = {
        let mut add = Add {
            io,
            storage,
            to_posix_eol,
            display_new,
            status: StatusLine::new(io),
            total: 0,
        };
        if io.metadata(&path)?.is_dir() {
            add.add_dir(&path)?
        } else {
            add.total = io.metadata(&path)?.len();
            add.add_file(&path)?
        }
    };
    io.stdout().println([k.as_str()])
}
