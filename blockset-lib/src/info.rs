use std::{collections::BTreeMap, io, ops::BitOrAssign};

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

fn get_dir<T: Io>(
    io: &T,
    path: &str,
    is_dir: bool,
    desired: Entry,
    entry: Entry,
    result: &mut Vec<(T::DirEntry, Entry)>,
) {
    if entry.0 & desired.0 == 0 {
        return;
    }
    result.extend(
        io.read_dir_type(&(CDT0.to_owned() + "/" + desired.dir() + path), is_dir)
            .unwrap_or_default()
            .into_iter()
            .map(|v| (v, entry)),
    );
}

fn get_all_dir<T: Io>(io: &T, path: &str, is_dir: bool, entry: Entry) -> Vec<(T::DirEntry, Entry)> {
    let mut result = Vec::default();
    get_dir(io, path, is_dir, ENTRY_ROOTS, entry, &mut result);
    get_dir(io, path, is_dir, ENTRY_PARTS, entry, &mut result);
    result
}

fn create_map(io: &impl Io, path: &str, is_dir: bool, e: Entry) -> EntryMap {
    let x = get_all_dir(io, path, is_dir, e);
    let mut map = EntryMap::default();
    for (de, e) in x {
        insert(&mut map, &de.path(), e);
    }
    map
}

pub fn calculate_total(io: &impl Io) -> io::Result<u64> {
    let stdout = &mut io.stdout();
    let mut total = 0;
    let state = &mut State::new(stdout);
    let a = create_map(io, "", true, ENTRY_ALL);
    let an = a.len() as u64;
    for (ai, (p, &e)) in a.iter().enumerate() {
        let b = create_map(io, p, true, e);
        let bn = b.len() as u64;
        for (bi, (p, &e)) in b.iter().enumerate() {
            let c = get_all_dir(io, p, false, e);
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

    /*
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
    */
}
