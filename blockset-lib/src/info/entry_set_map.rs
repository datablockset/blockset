use std::collections::BTreeMap;

use super::{entry_set::EntrySet, entry::Entry};

pub type EntrySetMap = BTreeMap<String, EntrySet>;

pub trait EntrySetMapEx {
    fn insert_entry(&mut self, file_name: &str, entry: Entry);
}

impl EntrySetMapEx for EntrySetMap {
    fn insert_entry(&mut self, file_name: &str, entry: Entry) {
        let es = entry.to_set();
        if let Some(e) = self.get_mut(file_name) {
            *e = e.union(es);
        } else {
            self.insert(file_name.to_owned(), es);
        }
    }
}
