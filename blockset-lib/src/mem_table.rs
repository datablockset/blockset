use std::{collections::HashMap, io};

use crate::{
    table::{Table, Type},
    uint::u224::U224,
};

pub type MemTable = [HashMap<U224, Vec<u8>>; 2];

impl Table for &mut MemTable {
    fn has_block(&self, t: Type, key: &U224) -> bool {
        self[t as usize].contains_key(key)
    }

    fn get_block(&self, t: Type, key: &U224) -> io::Result<Vec<u8>> {
        self[t as usize]
            .get(key)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }

    fn set_block(
        &mut self,
        t: Type,
        key: &U224,
        value: impl Iterator<Item = u8>,
    ) -> io::Result<()> {
        self[t as usize].insert(*key, value.collect());
        Ok(())
    }
}
