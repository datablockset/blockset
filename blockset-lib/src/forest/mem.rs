#![cfg(test)]
use std::{collections::BTreeMap, io};

use crate::uint::u224::U224;

use super::{node_id::ForestNodeId, Forest};

pub type MemForest = [BTreeMap<U224, Vec<u8>>; 2];

impl Forest for &mut MemForest {
    fn has_block(&self, id: &ForestNodeId) -> bool {
        self[id.t as usize].contains_key(&id.hash)
    }

    fn get_block(&self, id: &ForestNodeId) -> io::Result<Vec<u8>> {
        self[id.t as usize]
            .get(&id.hash)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "file not found"))
    }

    fn set_block(&mut self, id: &ForestNodeId, value: impl Iterator<Item = u8>) -> io::Result<()> {
        self[id.t as usize].insert(id.hash, value.collect());
        Ok(())
    }
}
