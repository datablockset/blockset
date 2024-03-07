use std::io;

use io_trait::{DirEntry, Io, Metadata};
use nanvm_lib::{
    common::cast::Cast,
    js::{any_cast::AnyCast, js_object::Property, js_string::new_string, new::New},
    mem::{global::GLOBAL, manager::Manager},
    serializer::to_json,
};

use crate::{app::invalid_input, common::print::Print};

fn read_dir_recursive<I: Io>(io: &I, path: &str) -> Vec<I::DirEntry> {
    io.read_dir(path)
        .unwrap()
        .into_iter()
        .flat_map(|s| {
            if s.metadata().unwrap().is_dir() {
                read_dir_recursive(io, s.path().as_str())
            } else {
                [s].cast()
            }
        })
        .collect()
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
    let list = read_dir_recursive(io, path);
    let list = list.into_iter().map(|s| property(GLOBAL, path.len(), s));
    let list = to_json(GLOBAL.new_js_object(list)).map_err(|_| invalid_input("to_json"));
    io.stdout().println(["add-dir: ", list?.as_str()])
}
