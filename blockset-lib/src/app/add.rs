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
    common::{print::Print, status_line::StatusLine},
};

use super::{invalid_input, read_to_tree, read_to_tree_file};

pub struct Add<'a, T: Io, S: 'a + TreeAdd, F: Fn(&'a T) -> S> {
    pub io: &'a T,
    pub storage: &'a F,
    pub to_posix_eol: bool,
    pub display_new: bool,
}

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

impl<'a, T: Io, S: 'a + TreeAdd, F: Fn(&'a T) -> S> Add<'a, T, S, F> {
    pub fn add_file(&self, state: &mut StatusLine<'a, T>, path: &str) -> io::Result<String> {
        read_to_tree_file(
            self.to_posix_eol,
            (self.storage)(self.io),
            self.io.open(path)?,
            self.io,
            state,
            self.display_new,
        )
    }
    fn path_to_json(&self, state: &mut StatusLine<'a, T>, path: &str) -> io::Result<String> {
        let files = read_dir_recursive(self.io, path)?;
        let mut list = Vec::default();
        for e in files {
            let f = e.path();
            let hash = self.add_file(state, f.as_str())?;
            list.push(property(GLOBAL, path.len(), f, hash));
        }
        dir_to_json(GLOBAL, list.into_iter())
    }
    pub fn add_dir(&self, state: &mut StatusLine<'a, T>, path: &str) -> io::Result<String> {
        let json = self.path_to_json(state, path)?;
        let mut state = StatusLine::new(self.io);
        read_to_tree(
            (self.storage)(self.io),
            Cursor::new(&json),
            self.io,
            &mut state,
            self.display_new,
        )
    }
}
