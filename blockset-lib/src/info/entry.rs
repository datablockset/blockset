use crate::
    file_table::{PARTS, ROOTS};

use super::entry_set::EntrySet;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Entry {
    Roots = 0,
    Parts = 1,
}

impl Entry {
    pub const fn dir(self) -> &'static str {
        match self {
            Entry::Roots => ROOTS,
            Entry::Parts => PARTS,
        }
    }
    pub const fn to_set(self) -> EntrySet {
        EntrySet::new(self)
    }
}
