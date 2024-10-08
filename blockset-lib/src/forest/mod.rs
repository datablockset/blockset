use std::io::{self, Write};

use crate::{
    cdt::{node_id::root, node_type::NodeType},
    uint::{u224::U224, u32::from_u8x4},
};

use self::node_id::ForestNodeId;

pub mod file;
pub mod mem;
pub mod node_id;
pub mod tree_add;

const EMPTY: U224 = root(&[0, 0]);

fn get_len(v: &[u8]) -> Option<usize> {
    let len = *v.first().unwrap();
    if len == 0x20 {
        None
    } else {
        Some(len as usize + 1)
    }
}

fn get_size(v: &[u8], len: usize, size: f64) -> (usize, f64) {
    let i = v.len();
    assert_eq!((i - len) % 28, 0);
    (i, size / ((i - len) / 28) as f64)
}

pub trait Forest {
    fn has_block(&self, id: &ForestNodeId) -> bool;
    fn get_block(&self, id: &ForestNodeId) -> io::Result<Vec<u8>>;
    fn set_block(&mut self, id: &ForestNodeId, value: impl Iterator<Item = u8>) -> io::Result<()>;
    fn check_set_block(
        &mut self,
        id: &ForestNodeId,
        value: impl Iterator<Item = u8>,
    ) -> io::Result<bool> {
        if self.has_block(id) {
            return Ok(false);
        }
        self.set_block(id, value)?;
        Ok(true)
    }
    // we should extract a state machine from the function and remove `set_progress`.
    fn restore(
        &self,
        id: &ForestNodeId,
        w: &mut impl Write,
        mut progress: impl FnMut(u64, f64) -> io::Result<()>,
    ) -> io::Result<u64> {
        if id.hash == EMPTY {
            return Ok(0);
        }
        let mut tail = Vec::default();
        let mut keys = [(id.hash, 1.0)].to_vec();
        let mut progress_p = 0.0;
        let mut progress_b = 0;
        let mut t = id.node_type;
        progress(0, 0.0)?;
        fn push_keys(
            len: usize,
            (mut i, size): (usize, f64),
            v: &[u8],
            keys: &mut Vec<([u32; 7], f64)>,
        ) {
            while len + 28 <= i {
                let mut kn = U224::default();
                i -= 28;
                let mut j = i;
                for ki in &mut kn {
                    let n = j + 4;
                    let slice = &v[j..n];
                    *ki = from_u8x4(slice.try_into().unwrap());
                    j = n;
                }
                keys.push((kn, size));
            }
        }
        while let Some((key, size)) = keys.pop() {
            let v = self.get_block(&ForestNodeId::new(t, &key))?;
            if let Some(len) = get_len(&v) {
                if len > 1 {
                    //assert!(tail.is_empty());
                    tail = v[1..len].to_vec();
                }
                push_keys(len, get_size(&v, len, size), &v, &mut keys);
            } else {
                let buf = &v[1..];
                w.write_all(buf)?;
                progress_p += size;
                progress_b += buf.len() as u64;
                progress(progress_b, progress_p)?;
            }
            t = NodeType::Child;
        }
        w.write_all(&tail)?;
        Ok(progress_b)
    }
}
