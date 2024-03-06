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

fn property<M: Manager>(m: M, e: impl DirEntry) -> Property<M::Dealloc> {
    let path = e
        .path()
        .replace('\\', "/")
        .encode_utf16()
        .collect::<Vec<_>>();
    let len = e.metadata().unwrap().len() as f64;
    (new_string(m, path).to_ref(), len.move_to_any())
}

pub fn add_dir<T: Io>(io: &T, mut a: T::Args) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing directory name"));
    let list = read_dir_recursive(io, path?.as_str());
    let list = list.into_iter().map(|s| property(GLOBAL, s));
    let list = to_json(GLOBAL.new_js_object(list)).map_err(|_| invalid_input("to_json"))?;
    io.stdout().println(["add-dir: ", list.as_str()])
}

#[cfg(test)]
mod test {
    use io_test::VirtualIo;
    use io_trait::Io;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::add_dir;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let io = VirtualIo::new(&["a"]);
        io.create_dir("a").unwrap();
        io.create("a/b.txt").unwrap();
        io.create("c.txt").unwrap();
        io.create("a/d.txt").unwrap();
        io.create_dir("a/e").unwrap();
        io.create("a/e/f.txt").unwrap();
        let mut a = io.args();
        a.next().unwrap();
        add_dir(&io, a).unwrap();
        let result = io.stdout.to_stdout();
        assert_eq!(result, "add-dir: {\"a/b.txt\":0,\"a/d.txt\":0,\"a/e/f.txt\":0}\n");
    }
}
