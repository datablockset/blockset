use crate::{cdt::node_type::NodeType, uint::u224::U224};

pub struct ForestNodeId {
    pub t: NodeType,
    pub hash: U224,
}

impl ForestNodeId {
    pub fn new(t: NodeType, hash: &U224) -> Self {
        Self { t, hash: *hash }
    }
}
