
use std::io::{self, Cursor};

use io_trait::Io;

use crate::{cdt::tree_add::TreeAdd, common::print::Print};

use super::{add::Add, read_to_tree};

pub fn add_dir<'a, T: Io, S: 'a + TreeAdd, F: Fn(&'a T) -> S>(
    add: &Add<'a, T, S, F>,
    path: &str,
) -> io::Result<()> {
    let json = add.path_to_json(path)?;
    let hash = read_to_tree(
        (add.storage)(add.io),
        Cursor::new(&json),
        add.io,
        add.display_new,
    )?;
    add.io.stdout().println([hash.as_str()])
}
