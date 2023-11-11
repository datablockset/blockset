use crate::{cdt::node_type::NodeType, uint::u224::U224};

pub struct ForestNodeId {
    pub node_type: NodeType,
    pub hash: U224,
}

impl ForestNodeId {
    pub fn new(node_type: NodeType, hash: &U224) -> Self {
        Self {
            node_type,
            hash: *hash,
        }
    }
}
