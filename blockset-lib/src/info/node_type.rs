use crate::file_table::{PARTS, ROOTS};

use super::node_type_set::NodeTypeSet;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum NodeType {
    Root = 0,
    Child = 1,
}

impl NodeType {
    pub const fn dir(self) -> &'static str {
        match self {
            NodeType::Root => ROOTS,
            NodeType::Child => PARTS,
        }
    }
    pub const fn to_set(self) -> NodeTypeSet {
        NodeTypeSet::new(self)
    }
}
