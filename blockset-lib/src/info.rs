use std::{collections::BTreeMap, io, ops::BitOrAssign};

use io_trait::{DirEntry, Io, Metadata};

use crate::{
    file_table::{CDT0, PARTS, ROOTS},
    state::{mb, State},
};

#[repr(u8)]
enum Entry {
    Roots = 0,
    Parts = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct EntrySet(u8);

const fn entry_set(t: Entry) -> EntrySet {
    EntrySet(1 << t as u8)
}

const fn union(a: EntrySet, b: EntrySet) -> EntrySet {
    EntrySet(a.0 | b.0)
}

const ENTRY_ROOTS: EntrySet = entry_set(Entry::Roots);
const ENTRY_PARTS: EntrySet = entry_set(Entry::Parts);
const ENTRY_ALL: EntrySet = union(ENTRY_PARTS, ENTRY_PARTS);

impl EntrySet {
    fn dir(&self) -> &str {
        if *self == ENTRY_ROOTS {
            ROOTS
        } else {
            PARTS
        }
    }
}

impl BitOrAssign for EntrySet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

type EntryMap = BTreeMap<String, EntrySet>;

fn insert(map: &mut EntryMap, file_name: &str, entry: EntrySet) {
    if let Some(e) = map.get_mut(file_name) {
        *e |= entry;
    } else {
        map.insert(file_name.to_owned(), entry);
    }
}

fn get_dir<T: Io>(
    io: &T,
    path: &str,
    is_dir: bool,
    desired: EntrySet,
    entry: EntrySet,
    result: &mut Vec<(T::DirEntry, EntrySet)>,
) {
    if entry.0 & desired.0 == 0 {
        return;
    }
    result.extend(
        io.read_dir_type(&(CDT0.to_owned() + "/" + desired.dir() + path), is_dir)
            .unwrap_or_default()
            .into_iter()
            .map(|v| (v, desired)),
    );
}

fn get_all_dir<T: Io>(io: &T, path: &str, is_dir: bool, entry: EntrySet) -> Vec<(T::DirEntry, EntrySet)> {
    let mut result = Vec::default();
    get_dir(io, path, is_dir, ENTRY_ROOTS, entry, &mut result);
    get_dir(io, path, is_dir, ENTRY_PARTS, entry, &mut result);
    result
}

fn file_name(path: &str) -> &str {
    path.rsplit_once('/').map(|(_, b)| b).unwrap_or(path)
}

fn create_map(io: &impl Io, path: &str, is_dir: bool, e: EntrySet) -> EntryMap {
    let x = get_all_dir(io, path, is_dir, e);
    let mut map = EntryMap::default();
    for (de, e) in x {
        insert(&mut map, file_name(&de.path()), e);
    }
    map
}

pub fn calculate_total(io: &impl Io) -> io::Result<u64> {
    let stdout = &mut io.stdout();
    let mut total = 0;
    let state = &mut State::new(stdout);
    let a = create_map(io, "", true, ENTRY_ALL);
    let an = a.len() as u64;
    for (ai, (af, &e)) in a.iter().enumerate() {
        let ap = "/".to_owned() + af;
        let b = create_map(io, &ap, true, e);
        let bn = b.len() as u64;
        for (bi, (bf, &e)) in b.iter().enumerate() {
            let c = get_all_dir(io, &(ap.to_owned() + "/" + bf), false, e);
            for (ic, _) in c.iter() {
                let d = ic.metadata()?.len();
                total += d;
            }
            let p = (bn * ai as u64 + bi as u64 + 1) as f64 / (an * bn) as f64;
            let e = total as f64 / p;
            let s = "size: ~".to_string()
                + &mb(e as u64)
                + ". "
                + &((p * 100.0) as u64).to_string()
                + "%.";
            state.set(&s)?;
        }
    }
    Ok(total)
}
