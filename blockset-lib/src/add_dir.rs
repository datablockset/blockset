use std::io;

use io_trait::Io;
use nanvm_lib::{js::new::New, mem::global::GLOBAL, serializer::WriteJson};

use crate::{app::invalid_input, common::print::Print};

pub fn add_dir<T: Io>(io: &T, mut a: T::Args) -> io::Result<()> {
    let path = a.next().ok_or(invalid_input("missing directory name"))?;
    let list = io.read_dir(path.as_str())?;
    let a = GLOBAL.new_js_array([]);
    let mut s = String::default();
    s.write_json(a).map_err(|_| invalid_input("write to JSON"))?;
    io.stdout().println(["add-dir: ", s.as_str()])?;
    Ok(())
}
