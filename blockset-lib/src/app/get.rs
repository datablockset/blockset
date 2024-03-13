use std::io::{self, Write};

use io_trait::Io;
use nanvm_lib::{
    js::any::Any,
    mem::manager::Manager,
    parser::{parse_with_tokens, Context},
    tokenizer::tokenize,
};

use crate::{
    cdt::node_type::NodeType,
    forest::{file::FileForest, node_id::ForestNodeId, Forest},
    uint::u224::U224,
};

pub fn restore(io: &impl Io, hash: &U224, w: &mut impl Write) -> io::Result<()> {
    FileForest(io).restore(&ForestNodeId::new(NodeType::Root, hash), w, io)
}

pub fn parse_json<M: Manager>(io: &impl Io, manager: M, v: Vec<u8>) -> io::Result<Any<M::Dealloc>> {
    let s = String::from_utf8(v)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
    let tokens = tokenize(s);
    let result = parse_with_tokens(
        &Context::new(manager, io, String::default()),
        tokens.into_iter(),
    );
    let result = result.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON"))?;
    Ok(result.any)
}
