use std::collections::BTreeMap;

use crate::cdt::node_type::NodeType;

use super::node_type_set::NodeTypeSet;

pub type DirEntryMap = BTreeMap<String, NodeTypeSet>;

pub trait DirEntryMapEx {
    fn insert_dir_entry(&mut self, dir_entry: &str, entry: NodeType);
}

impl DirEntryMapEx for DirEntryMap {
    fn insert_dir_entry(&mut self, dir_entry: &str, entry: NodeType) {
        let es = NodeTypeSet::new(entry);
        if let Some(e) = self.get_mut(dir_entry) {
            *e = e.union(es);
        } else {
            self.insert(dir_entry.to_owned(), es);
        }
    }
}
