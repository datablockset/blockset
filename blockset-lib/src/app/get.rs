use std::io::{self, Cursor, Write};

use io_trait::Io;
use nanvm_lib::{
    common::default::default,
    js::{any::Any, js_object::JsObjectRef},
    mem::{global::GLOBAL, manager::Manager},
    parser::{parse_with_tokens, Context, ParseError, ParseResult},
    tokenizer::tokenize,
};

use crate::{
    cdt::node_type::NodeType,
    common::status_line::{mb, StatusLine},
    forest::{file::FileForest, node_id::ForestNodeId, Forest},
    uint::u224::U224,
};

use super::{add::posix_path, get_hash, invalid_input, js_string_to_string, str_to_hash, try_move};

pub fn restore(
    io: &impl Io,
    hash: &U224,
    w: &mut impl Write,
    progress: &mut impl FnMut(u64, f64) -> io::Result<()>,
) -> io::Result<u64> {
    FileForest(io).restore(&ForestNodeId::new(NodeType::Root, hash), w, progress)
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

fn set_progress(
    state: &mut StatusLine<impl Io>,
    progress_b: u64,
    progress_p: f64,
) -> io::Result<()> {
    state.set_progress(&(mb(progress_b) + ", "), progress_p)
}

fn get_if(d: &U224, path: &str, io: &impl Io) -> io::Result<()> {
    let mut state = StatusLine::new(io);
    if path.ends_with('/') {
        let mut buffer = Vec::default();
        let mut w = Cursor::new(&mut buffer);
        restore(io, d, &mut w, &mut |_, _| Ok(()))?;
        let json = try_move::<_, JsObjectRef<_>>(parse_json(io, GLOBAL, buffer)?)?;
        let items = json.items();
        let t = items.len();
        let mut b = 0;
        for (offset, (k, v)) in items.iter().enumerate() {
            let file = js_string_to_string(k)?;
            let hash = js_string_to_string(&try_move(v.clone())?)?;
            b += restore(
                io,
                &str_to_hash(&hash)?,
                &mut create_file_recursively(io, (path.to_owned() + &file).as_str())?,
                &mut |progress_b, progress_p| {
                    set_progress(
                        &mut state,
                        b + progress_b,
                        (offset as f64 + progress_p) / t as f64,
                    )
                },
            )?;
        }
        Ok(())
    } else {
        restore(
            io,
            d,
            &mut create_file_recursively(io, path)?,
            &mut |progress_b, progress_p| set_progress(&mut state, progress_b, progress_p),
        )?;
        Ok(())
    }
}

pub fn get<T: Io>(io: &T, a: &mut T::Args) -> io::Result<()> {
    get_if(
        &get_hash(a)?,
        &posix_path(a.next().ok_or(invalid_input("missing file name"))?.as_str()),
        io,
    )
}
