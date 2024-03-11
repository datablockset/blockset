use core::ops::Deref;
use std::io;

use io_trait::{DirEntry, Io, Metadata};
use nanvm_lib::{
    common::{cast::Cast, default::default},
    js::{
        any_cast::AnyCast,
        js_object::Property,
        js_string::{new_string, JsStringRef},
        new::New,
    },
    mem::{global::GLOBAL, manager::Manager},
    serializer::to_json,
};

use crate::{app::invalid_input, cdt::tree_add::TreeAdd, common::print::Print};

use super::add_file::add_file;

fn read_dir_recursive<I: Io>(io: &I, path: &str) -> io::Result<Vec<I::DirEntry>> {
    let mut result: Vec<_> = default();
    let mut dirs = [path.to_owned()].cast();
    while let Some(dir) = dirs.pop() {
        for entry in io.read_dir(dir.as_str())? {
            if entry.metadata()?.is_dir() {
                dirs.push(entry.path().to_owned());
            } else {
                result.push(entry);
            }
        }
    }
    Ok(result)
}

fn str_to_js_string<M: Manager>(m: M, s: impl Deref<Target = str>) -> JsStringRef<M::Dealloc> {
    new_string(m, s.encode_utf16().collect::<Vec<_>>()).to_ref()
}

fn property<M: Manager>(
    m: M,
    path_len: usize,
    file: impl Deref<Target = str>,
    hash: impl Deref<Target = str>,
) -> Property<M::Dealloc> {
    (
        str_to_js_string(m, file[path_len + 1..].replace('\\', "/")),
        str_to_js_string(m, hash).move_to_any(),
    )
}

fn dir_to_json<M: Manager>(
    m: M,
    list: impl ExactSizeIterator<Item = Property<M::Dealloc>>,
) -> io::Result<String> {
    to_json(m.new_js_object(list)).map_err(|_| invalid_input("to_json"))
}

fn path_to_json<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    to_posix_eol: bool,
    storage: &impl Fn(&'a T) -> S,
    display_new: bool,
    path: &str,
) -> io::Result<String> {
    let files = read_dir_recursive(io, path)?;
    let mut list = Vec::default();
    for e in &files {
        let f = e.path();
        let hash = add_file(io, f.as_str(), to_posix_eol, storage, display_new)?;
        list.push(property(GLOBAL, path.len(), f, hash));
    }
    dir_to_json(GLOBAL, list.into_iter())
}

pub fn add_dir<'a, T: Io, S: 'a + TreeAdd>(
    io: &'a T,
    to_posix_eol: bool,
    storage: &impl Fn(&'a T) -> S,
    display_new: bool,
    path: &str,
) -> io::Result<()> {
    let json = path_to_json(io, to_posix_eol, storage, display_new, path)?;
    io.stdout().println(["add-dir: ", json.as_str()])
}
