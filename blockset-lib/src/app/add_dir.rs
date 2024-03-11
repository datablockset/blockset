use std::io;

use io_trait::{DirEntry, Io, Metadata};
use nanvm_lib::{
    common::{cast::Cast, default::default},
    js::{any_cast::AnyCast, js_object::Property, js_string::new_string, new::New},
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

fn property<M: Manager>(m: M, path_len: usize, e: impl DirEntry) -> Property<M::Dealloc> {
    let path = e.path()[path_len + 1..]
        .replace('\\', "/")
        .encode_utf16()
        .collect::<Vec<_>>();
    let len = e.metadata().unwrap().len() as f64;
    (new_string(m, path).to_ref(), len.move_to_any())
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
    for e in &files {
        add_file(io, e.path().as_str(), to_posix_eol, storage, display_new)?;
    }
    dir_to_json(
        GLOBAL,
        files.into_iter().map(|s| property(GLOBAL, path.len(), s)),
    )
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
