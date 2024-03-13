use std::io::{self, Write};

use io_trait::Io;

use crate::{
    cdt::node_type::NodeType,
    forest::{file::FileForest, node_id::ForestNodeId, Forest},
    uint::u224::U224,
};

pub fn restore(io: &impl Io, hash: &U224, w: &mut impl Write) -> io::Result<()> {
    FileForest(io).restore(&ForestNodeId::new(NodeType::Root, hash), w, io)
}
