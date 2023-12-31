use crate::cdt::node_type::NodeType;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct NodeTypeSet(u8);

impl NodeTypeSet {
    const EMPTY: NodeTypeSet = NodeTypeSet(0);
    const ROOT: NodeTypeSet = NodeTypeSet::new(NodeType::Root);
    const CHILD: NodeTypeSet = NodeTypeSet::new(NodeType::Child);
    pub const ALL: NodeTypeSet = Self::ROOT.union(Self::CHILD);
    const fn eq(self, b: NodeTypeSet) -> bool {
        self.0 == b.0
    }
    pub const fn new(v: NodeType) -> NodeTypeSet {
        NodeTypeSet(1 << v as u8)
    }
    pub const fn union(self, b: NodeTypeSet) -> NodeTypeSet {
        NodeTypeSet(self.0 | b.0)
    }
    const fn intersection(self, b: NodeTypeSet) -> NodeTypeSet {
        NodeTypeSet(self.0 & b.0)
    }
    pub const fn has(self, b: NodeType) -> bool {
        !self.intersection(NodeTypeSet::new(b)).eq(Self::EMPTY)
    }
}
