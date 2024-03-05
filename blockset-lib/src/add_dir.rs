use std::io;

use io_trait::Io;
use nanvm_lib::{js::new::New, mem::global::GLOBAL, serializer::WriteJson};

use crate::{app::invalid_input, common::print::Print};

pub fn add_dir<T: Io>(io: &T, mut a: T::Args) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing directory name"))?;
    let list = io.read_dir(path.as_str())?;
    let a = GLOBAL.new_js_array(
        list.into_iter().map(|_| GLOBAL.new_js_string([])));
    let mut s = String::default();
    s.write_json(a)
        .map_err(|_| invalid_input("write to JSON"))?;
    io.stdout().println(["add-dir: ", s.as_str()])?;
    Ok(())
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
        let mut a = io.args();
        a.next().unwrap();
        add_dir(&io, a).unwrap();
        let result = io.stdout.to_stdout();
        assert_eq!(result, "add-dir: []\n");
    }
}
