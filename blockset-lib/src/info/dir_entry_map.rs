use std::collections::BTreeMap;

use super::{node_type::NodeType, node_type_set::NodeTypeSet};

pub type DirEntryMap = BTreeMap<String, NodeTypeSet>;

pub trait DirEntryMapEx {
    fn insert_dir_entry(&mut self, dir_entry: &str, entry: NodeType);
}

impl DirEntryMapEx for DirEntryMap {
    fn insert_dir_entry(&mut self, dir_entry: &str, entry: NodeType) {
        let es = entry.to_set();
        if let Some(e) = self.get_mut(dir_entry) {
            *e = e.union(es);
        } else {
            self.insert(dir_entry.to_owned(), es);
        }
    }
}
