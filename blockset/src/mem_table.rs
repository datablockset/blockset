use std::collections::HashMap;

use crate::{table::{Table, Type}, u224::U224};

pub type MemTable = [HashMap<U224, Vec<u8>>; 2];

impl Table for MemTable {
    fn has_block(&self, t: Type, key: &U224) -> bool {
        self[t as usize].contains_key(key)
    }

    fn get_block(&self, t: Type, key: &U224) -> Option<Vec<u8>> {
        self[t as usize].get(key).cloned()
    }

    fn set_block(&mut self, t: Type, key: &U224, value: impl Iterator<Item = u8>) -> Option<()> {
        self[t as usize].insert(*key, value.collect());
        Some(())
    }
}
