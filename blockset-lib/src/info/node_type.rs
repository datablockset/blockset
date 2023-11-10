use crate::{
    cdt::node_type::NodeType,
    forest::file::{PARTS, ROOTS},
};

pub const fn dir(t: NodeType) -> &'static str {
    match t {
        NodeType::Root => ROOTS,
        NodeType::Child => PARTS,
    }
}
