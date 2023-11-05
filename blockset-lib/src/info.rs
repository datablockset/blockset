use std::{io, collections::BTreeMap, ops::BitOrAssign};

use io_trait::{DirEntry, Io, Metadata};

use crate::{
    file_table::{CDT0, PARTS, ROOTS},
    state::{mb, State},
};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Entry(u8);

const ENTRY_ROOTS: Entry = Entry(1);
const ENTRY_PARTS: Entry = Entry(2);
const ENTRY_ALL: Entry = Entry(3);

impl Entry {
    fn dir(&self) -> &str {
        if *self == ENTRY_ROOTS {
            ROOTS
        } else {
            PARTS
        }
    }
}

impl BitOrAssign for Entry {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

type EntryMap = BTreeMap<String, Entry>;

fn insert(map: &mut EntryMap, path: &str, entry: Entry) {
    if let Some(e) = map.get_mut(path) {
        *e |= entry;
    } else {
        map.insert(path.to_owned(), entry);
    }
}

fn insert_range(map: &mut EntryMap, i: impl Iterator<Item = String>, entry: Entry) {
    for j in i {
        insert(map, &j, entry);
    }
}

fn insert_dir(map: &mut EntryMap, io: &impl Io, path:&str,  is_dir: bool, desired: Entry, entry: Entry) -> () {
    if entry.0 & desired.0 == 0 {
        return;
    }
    let v = io.read_dir_type(&(CDT0.to_owned() + "/" + entry.dir() + path), is_dir).unwrap_or_default();
    insert_range(map, v.iter().map(|x| x.path()), entry);
}

fn create_map(io: &impl Io, path: &str, is_dir: bool, e: Entry) -> EntryMap {
    let mut map = EntryMap::default();
    insert_dir(&mut map, io, path, is_dir, ENTRY_ROOTS, e);
    insert_dir(&mut map, io, path, is_dir, ENTRY_PARTS, e);
    map
}

pub fn calculate_total(io: &impl Io) -> io::Result<u64> {
    let stdout = &mut io.stdout();
    let map = create_map(io, "", true, ENTRY_ALL);

    //
    let f = |d| {
        io.read_dir_type(&(CDT0.to_owned() + "/" + d), true)
            .unwrap_or_default()
    };
    let mut a = f(ROOTS);
    a.extend(f(PARTS));
    let an = a.len() as u64;
    let mut total = 0;
    let state = &mut State::new(stdout);
    for (ai, ia) in a.iter().enumerate() {
        let b = io.read_dir_type(&ia.path(), true)?;
        let bn = b.len() as u64;
        for (bi, ib) in b.iter().enumerate() {
            let c = io.read_dir_type(&ib.path(), false)?;
            for ic in c.iter() {
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
