use std::io;

use io_trait::{DirEntry, Io};
use nanvm_lib::{js::new::New, mem::global::GLOBAL, serializer::to_json};

use crate::{app::invalid_input, common::print::Print};

pub fn add_dir<T: Io>(io: &T, mut a: T::Args) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing directory name"))?;
    let list = io.read_dir(path.as_str())?;
    let a = GLOBAL.new_js_array(list.into_iter().map(|s| {
        let s16 = s.path().encode_utf16().collect::<Vec<_>>();
        GLOBAL.new_js_string(s16)
    }));
    io.stdout().println([
        "add-dir: ",
        to_json(a).map_err(|_| invalid_input("to_json"))?.as_str(),
    ])
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
        let mut a = io.args();
        a.next().unwrap();
        add_dir(&io, a).unwrap();
        let result = io.stdout.to_stdout();
        assert_eq!(result, "add-dir: [\"a/b.txt\",\"a/d.txt\"]\n");
    }
}
