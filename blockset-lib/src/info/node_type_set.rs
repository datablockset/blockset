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

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::NodeTypeSet;
    use crate::cdt::node_type::NodeType;

    #[inline(never)]
    fn check(x: NodeType, y: NodeType, union: fn(NodeTypeSet, NodeTypeSet) -> NodeTypeSet) {
        let xi = 1 << x as u8;
        let yi = 1 << y as u8;
        let xs = NodeTypeSet::new(x);
        let ys = NodeTypeSet::new(y);
        assert_eq!(union(xs, ys).0, xi | yi);
        assert_eq!(xs.intersection(ys).0, xi & yi);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        check(NodeType::Child, NodeType::Child, NodeTypeSet::union);
        check(NodeType::Child, NodeType::Root, NodeTypeSet::union);
        let x = NodeTypeSet::new(NodeType::Root);
        assert_eq!(x.0, 1);
        let y = NodeTypeSet::new(NodeType::Child);
        assert_eq!(y.0, 2);
        let z = x.union(y);
        assert_eq!(z.0, 3);
        assert_eq!(z.has(NodeType::Root), true);
        assert_eq!(z.has(NodeType::Child), true);
        assert_eq!(z.has(NodeType::Root), true);
        assert_eq!(z.has(NodeType::Child), true);
    }
}
