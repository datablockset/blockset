use super::entry::Entry;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct EntrySet(u8);

impl EntrySet {
    const EMPTY: EntrySet = EntrySet(0);
    const ROOTS: EntrySet = Entry::Roots.to_set();
    const PARTS: EntrySet = Entry::Parts.to_set();
    pub const ALL: EntrySet = Self::ROOTS.union(Self::PARTS);
    const fn eq(self, b: EntrySet) -> bool {
        self.0 == b.0
    }
    pub const fn new(v: Entry) -> EntrySet {
        EntrySet(1 << v as u8)
    }
    pub const fn union(self, b: EntrySet) -> EntrySet {
        EntrySet(self.0 | b.0)
    }
    const fn intersection(self, b: EntrySet) -> EntrySet {
        EntrySet(self.0 & b.0)
    }
    pub const fn has(self, b: Entry) -> bool {
        !self.intersection(b.to_set()).eq(Self::EMPTY)
    }
}
