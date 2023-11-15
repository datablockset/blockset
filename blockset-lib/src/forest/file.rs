use std::io;

use io_trait::Io;

use crate::{cdt::node_type::NodeType, common::base32::ToBase32, forest::Forest};

use super::node_id::ForestNodeId;

pub struct FileForest<'a, T: Io>(pub &'a T);

pub const CDT0: &str = "cdt0";

pub const ROOTS: &str = "roots";

pub const PARTS: &str = "parts";

pub const fn dir(t: NodeType) -> &'static str {
    [ROOTS, PARTS][t as usize]
}

fn path(id: &ForestNodeId) -> String {
    let s = id.hash.to_base32();
    CDT0.to_owned() + "/" + dir(id.node_type) + "/" + &s[..2] + "/" + &s[2..4] + "/" + &s[4..]
}

impl<'a, T: Io> Forest for FileForest<'a, T> {
    fn has_block(&self, id: &ForestNodeId) -> bool {
        self.0.metadata(&path(id)).is_ok()
    }

    fn get_block(&self, id: &ForestNodeId) -> io::Result<Vec<u8>> {
        self.0.read(&path(id))
    }

    fn set_block(&mut self, id: &ForestNodeId, value: impl Iterator<Item = u8>) -> io::Result<()> {
        let x = value.collect::<Vec<_>>();
        let p = path(id);
        self.0.write_recursively(&p, &x)
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{cdt::node_type::NodeType, forest::node_id::ForestNodeId};

    use super::path;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let k = [
            0x0ae63892, 0xc81cd1b0, 0x4f97a944, 0x891a80e6, 0x9205f2b7, 0xc9d3c292, 0x397b08b5,
        ];
        path(&ForestNodeId::new(NodeType::Root, &k));
    }
}
