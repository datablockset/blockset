mod dir_entry_map;
mod node_type_set;

use std::io;

use io_trait::{DirEntry, Io, Metadata};

use crate::{
    cdt::node_type::NodeType,
    forest::file::{dir, CDT0},
    common::state::{mb, State},
};

use self::{
    dir_entry_map::{DirEntryMap, DirEntryMapEx},
    node_type_set::NodeTypeSet,
};

fn get_dir<T: Io>(
    io: &T,
    path: &str,
    is_dir: bool,
    desired: NodeType,
    entry: NodeTypeSet,
    result: &mut Vec<(T::DirEntry, NodeType)>,
) {
    if !entry.has(desired) {
        return;
    }
    result.extend(
        io.read_dir_type(&(CDT0.to_owned() + "/" + dir(desired) + path), is_dir)
            .unwrap_or_default()
            .into_iter()
            .map(|v| (v, desired)),
    );
}

fn get_all_dir<T: Io>(
    io: &T,
    path: &str,
    is_dir: bool,
    entry: NodeTypeSet,
) -> Vec<(T::DirEntry, NodeType)> {
    let mut result = Vec::default();
    get_dir(io, path, is_dir, NodeType::Root, entry, &mut result);
    get_dir(io, path, is_dir, NodeType::Child, entry, &mut result);
    result
}

fn file_name(path: &str) -> &str {
    path.rsplit_once('/').map(|(_, b)| b).unwrap_or(path)
}

fn create_map(io: &impl Io, path: &str, is_dir: bool, e: NodeTypeSet) -> DirEntryMap {
    let x = get_all_dir(io, path, is_dir, e);
    let mut map = DirEntryMap::default();
    for (de, e) in x {
        map.insert_dir_entry(file_name(&de.path()), e);
    }
    map
}

pub fn calculate_total(io: &impl Io) -> io::Result<u64> {
    let mut total = 0;
    let state = &mut State::new(io);
    let a = create_map(io, "", true, NodeTypeSet::ALL);
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
            let s = "Estimated size: ~".to_string() + &mb(e as u64) + ", ";
            state.set_progress(&s, p)?;
        }
    }
    Ok(total)
}
