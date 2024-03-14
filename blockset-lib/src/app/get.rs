use std::io::{self, Write};

use io_trait::Io;
use nanvm_lib::{
    js::any::Any,
    mem::manager::Manager,
    parser::{parse_with_tokens, Context, ParseError, ParseResult},
    tokenizer::tokenize,
};

use crate::{
    cdt::node_type::NodeType,
    forest::{file::FileForest, node_id::ForestNodeId, Forest},
    uint::u224::U224,
};

use super::invalid_input;

pub fn restore(io: &impl Io, hash: &U224, w: &mut impl Write) -> io::Result<()> {
    FileForest(io).restore(&ForestNodeId::new(NodeType::Root, hash), w, io)
}

fn tokenize_and_parse<M: Manager>(
    io: &impl Io,
    manager: M,
    s: String,
) -> Result<ParseResult<M>, ParseError> {
    parse_with_tokens(
        &Context::new(manager, io, String::default()),
        tokenize(s).into_iter(),
    )
}

pub fn parse_json<M: Manager>(io: &impl Io, manager: M, v: Vec<u8>) -> io::Result<Any<M::Dealloc>> {
    let s = String::from_utf8(v).map_err(|_| invalid_input("Invalid UTF-8"))?;
    let result = tokenize_and_parse(io, manager, s).map_err(|_| invalid_input("Invalid JSON"))?;
    Ok(result.any)
}

pub fn create_file_recursivly<T: Io>(io: &T, path: &str) -> io::Result<T::File> {
    if let Some((p, _)) = path.rsplit_once('/') {
        io.create_dir_recursively(p)?;
    }
    io.create(path)
}
