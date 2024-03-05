use std::io;

use io_trait::{DirEntry, Io, Metadata};
use nanvm_lib::{js::new::New, mem::global::GLOBAL, serializer::to_json};

use crate::{app::invalid_input, common::print::Print};

pub fn add_dir<T: Io>(io: &T, mut a: T::Args) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing directory name"))?;
    let list = io.read_dir(path.as_str())?;
    let list = list
        .into_iter()
        .flat_map(|s| {
            if s.metadata().unwrap().is_dir() {
                Vec::default() // GLOBAL.new_js_string([])
            } else {
                let s16 = s.path().encode_utf16().collect::<Vec<_>>();
                [GLOBAL.new_js_string(s16)].to_vec()
            }
        })
        .collect::<Vec<_>>();
    let a = GLOBAL.new_js_array(list);
    let list = to_json(a).map_err(|_| invalid_input("to_json"))?;
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
        assert_eq!(result, "add-dir: [\"a/b.txt\",\"a/d.txt\"]\n");
    }
}
