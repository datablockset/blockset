use core::ops::Deref;
use std::io::{self, Cursor};

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

use crate::{
    cdt::tree_add::TreeAdd,
    common::{progress::State, status_line::StatusLine},
};

use super::{invalid_input, read_to_tree, read_to_tree_file};

pub struct Add<'a, T: Io, S: 'a + TreeAdd, F: Fn(&'a T) -> S> {
    pub io: &'a T,
    pub storage: &'a F,
    pub to_posix_eol: bool,
    pub display_new: bool,
    pub status: StatusLine<'a, T>,
    pub p: State,
}

pub fn posix_path(s: &str) -> String {
    s.replace('\\', "/")
}

fn read_dir_recursive<I: Io>(io: &I, path: &str) -> io::Result<Vec<(String, u64)>> {
    let mut result: Vec<_> = default();
    let mut dirs = [path.to_owned()].cast();
    while let Some(dir) = dirs.pop() {
        for entry in io.read_dir(dir.as_str())? {
            let m = entry.metadata()?;
            if m.is_dir() {
                dirs.push(entry.path());
            } else {
                result.push((posix_path(&entry.path()[path.len() + 1..]), m.len()));
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
    file: impl Deref<Target = str>,
    hash: impl Deref<Target = str>,
) -> Property<M::Dealloc> {
    (
        str_to_js_string(m, file),
        str_to_js_string(m, hash).move_to_any(),
    )
}

fn dir_to_json<M: Manager>(
    m: M,
    list: impl ExactSizeIterator<Item = Property<M::Dealloc>>,
) -> io::Result<String> {
    to_json(m.new_js_object(list)).map_err(|_| invalid_input("to_json"))
}

impl<'a, T: Io, S: 'a + TreeAdd, F: Fn(&'a T) -> S> Add<'a, T, S, F> {
    pub fn add_file(&mut self, path: &str) -> io::Result<String> {
        read_to_tree_file(
            self.to_posix_eol,
            (self.storage)(self.io),
            self.io.open(path)?,
            &mut self.status,
            self.display_new,
            self.p,
        )
    }
    fn path_to_json(&mut self, path: &str) -> io::Result<String> {
        let files = read_dir_recursive(self.io, path)?;
        let mut json_len = 1;
        // JSON size:
        // `{` +
        // `"` + path + `":"` + 45 + `",` = path.len() + 51
        for (path, len) in &files {
            self.p.total += len;
            json_len += path.len() + 51;
        }
        self.p.total += json_len as u64;
        let mut list = Vec::default();
        for (p, len) in files {
            let hash = self.add_file((path.to_owned() + "/" + &p).as_str())?;
            list.push(property(GLOBAL, p, hash));
            self.p.current += len;
        }
        let json = dir_to_json(GLOBAL, list.into_iter())?;
        assert_eq!(json.len(), json_len);
        Ok(json)
    }
    pub fn add_dir(&mut self, path: &str) -> io::Result<String> {
        read_to_tree(
            (self.storage)(self.io),
            Cursor::new(self.path_to_json(path)?),
            &mut self.status,
            self.display_new,
            self.p,
        )
    }
}
