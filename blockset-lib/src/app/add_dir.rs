use std::io;

use io_trait::{DirEntry, Io, Metadata};
use nanvm_lib::{
    common::{cast::Cast, default::default},
    js::{any_cast::AnyCast, js_object::Property, js_string::new_string, new::New},
    mem::{global::GLOBAL, manager::Manager},
    serializer::to_json,
};

use crate::{app::invalid_input, common::print::Print};

fn read_dir_recursive<I: Io>(io: &I, path: &str) -> io::Result<Vec<I::DirEntry>> {
    let mut result: Vec<_> = default();
    let mut dirs = [path.to_owned()].cast();
    while let Some(dir) = dirs.pop() {
        for entry in io.read_dir(dir.as_str())? {
            if entry.metadata().unwrap().is_dir() {
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

pub fn add_dir<T: Io>(io: &T, path: &str) -> io::Result<()> {
    let list = read_dir_recursive(io, path)?;
    let list = list.into_iter().map(|s| property(GLOBAL, path.len(), s));
    let list = to_json(GLOBAL.new_js_object(list)).map_err(|_| invalid_input("to_json"));
    io.stdout().println(["add-dir: ", list?.as_str()])
}