#![cfg(test)]
use std::{collections::BTreeMap, io};

use crate::{cdt::node_type::NodeType, forest::Forest, uint::u224::U224};

pub type MemForest = [BTreeMap<U224, Vec<u8>>; 2];

impl Forest for &mut MemForest {
    fn has_block(&self, t: NodeType, hash: &U224) -> bool {
        self[t as usize].contains_key(hash)
    }

    fn get_block(&self, t: NodeType, key: &U224) -> io::Result<Vec<u8>> {
        self[t as usize]
            .get(key)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }

    fn set_block(
        &mut self,
        t: NodeType,
        key: &U224,
        value: impl Iterator<Item = u8>,
    ) -> io::Result<()> {
        self[t as usize].insert(*key, value.collect());
        Ok(())
    }
}
