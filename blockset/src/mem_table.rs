use std::collections::HashMap;

use crate::{u224::U224, table::Table};

pub type MemTable = HashMap<U224, Vec<u8>>;

impl Table for MemTable {
    fn has_block(&self, key: &U224) -> bool {
        self.contains_key(key)
    }

    fn get_block(&self, key: &U224) -> Option<Vec<u8>> {
        self.get(key).cloned()
    }

    fn set_block(&mut self, key: &U224, value: impl Iterator<Item = u8>) {
        self.insert(*key, value.collect());
    }
}

impl Table for &mut MemTable {
    fn has_block(&self, key: &U224) -> bool {
        self.contains_key(key)
    }

    fn get_block(&self, key: &U224) -> Option<Vec<u8>> {
        self.get(key).cloned()
    }

    fn set_block(&mut self, key: &U224, value: impl Iterator<Item = u8>) {
        self.insert(*key, value.collect());
    }
}