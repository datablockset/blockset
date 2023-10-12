use std::marker::PhantomData;

use crate::{Io, table::Table, u224::U224, base32::ToBase32, Tables};

pub trait Path {
    const PREFIX: &'static str;
}

pub struct FileTable<'a, T: Io, P: Path>(&'a mut T, PhantomData<P>);

struct MainPath();

impl Path for MainPath {
    const PREFIX: &'static str = "";
}

pub type MainFileTable<'a, T> = FileTable<'a, T, MainPath>;

struct PartsPath();

impl Path for PartsPath {
    const PREFIX: &'static str = "_";
}

pub type PartsFileTable<'a, T> = FileTable<'a, T, PartsPath>;

const DIR: &str = "cdt0/";

fn path<P: Path>(key: &U224) -> String {
    DIR.to_owned() + P::PREFIX + &key.to_base32()
}

impl<'a, T: Io, P: Path> Table for FileTable<'a, T, P> {
    fn has_block(&self, key: &U224) -> bool {
        self.0.metadata(&path::<P>(key)).is_ok()
    }

    fn get_block(&self, key: &U224) -> Option<Vec<u8>> {
        self.0.read(&path::<P>(key)).ok()
    }

    fn set_block(&mut self, key: &U224, value: impl Iterator<Item = u8>) -> Option<()> {
        self.0.write(&path::<P>(key), &value.collect::<Vec<_>>()).ok()
    }
}