use std::io::{self, Write};

use io_trait::Io;
use nanvm_lib::{
    common::default::default,
    js::any::Any,
    mem::manager::Manager,
    parser::{parse_with_tokens, Context, ParseError, ParseResult},
    tokenizer::tokenize,
};

use crate::{
    cdt::node_type::NodeType,
    common::status_line::{mb, StatusLine},
    forest::{file::FileForest, node_id::ForestNodeId, Forest},
    uint::u224::U224,
};

use super::invalid_input;

pub fn restore(io: &impl Io, hash: &U224, w: &mut impl Write) -> io::Result<()> {
    let mut state = StatusLine::new(io);
    FileForest(io).restore(
        &ForestNodeId::new(NodeType::Root, hash),
        w,
        |progress_b, progress_p| state.set_progress(&(mb(progress_b) + ", "), progress_p),
    )
}

fn tokenize_and_parse<M: Manager>(
    io: &impl Io,
    manager: M,
    s: String,
) -> Result<ParseResult<M>, ParseError> {
    parse_with_tokens(
        &mut Context::new(manager, io, default(), &mut default()),
        tokenize(s).into_iter(),
    )
}

pub fn parse_json<M: Manager>(io: &impl Io, manager: M, v: Vec<u8>) -> io::Result<Any<M::Dealloc>> {
    let s = String::from_utf8(v).map_err(|_| invalid_input("Invalid UTF-8"))?;
    let result = tokenize_and_parse(io, manager, s).map_err(|_| invalid_input("Invalid JSON"))?;
    Ok(result.any)
}

fn dir(path: &str) -> Option<&str> {
    path.rsplit_once('/').map(|(d, _)| d)
}

fn create_file_path_recursively<T: Io>(io: &T, path: &str) -> io::Result<()> {
    dir(path).map(|d| io.create_dir_recursively(d));
    Ok(())
}

pub fn create_file_recursively<T: Io>(io: &T, path: &str) -> io::Result<T::File> {
    create_file_path_recursively(io, path)?;
    io.create(path)
}
