use crate::{Io, table::{Table, Type}, u224::U224, base32::ToBase32};

pub struct FileTable<'a, T: Io>(pub &'a mut T);

const DIR: &str = "cdt0/";

fn path(t: Type, key: &U224) -> String {
    DIR.to_owned() + ["", "_"][t as usize] + &key.to_base32()
}

impl<'a, T: Io> Table for FileTable<'a, T> {
    fn has_block(&self, t: Type, key: &U224) -> bool {
        self.0.metadata(&path(t, key)).is_ok()
    }

    fn get_block(&self, t: Type, key: &U224) -> Option<Vec<u8>> {
        self.0.read(&path(t, key)).ok()
    }

    fn set_block(&mut self, t: Type, key: &U224, value: impl Iterator<Item = u8>) -> Option<()> {
        self.0.write(&path(t, key), &value.collect::<Vec<_>>()).ok()
    }
}