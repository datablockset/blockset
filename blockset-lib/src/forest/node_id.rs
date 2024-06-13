use crate::{cdt::node_type::NodeType, uint::u224x::U224};

pub struct ForestNodeId {
    pub node_type: NodeType,
    pub hash: U224,
}

impl ForestNodeId {
    pub const fn new(node_type: NodeType, hash: &U224) -> Self {
        Self {
            node_type,
            hash: *hash,
        }
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::ForestNodeId;
    use crate::cdt::node_type::NodeType;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let x = ForestNodeId::new(NodeType::Root, &[0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(x.node_type, NodeType::Root);
        assert_eq!(x.hash, [0, 0, 0, 0, 0, 0, 0]);
    }
}
