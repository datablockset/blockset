use std::io;

use io_trait::Io;

use crate::{
    base32::ToBase32,
    forest::forest::{Forest, Type},
    uint::u224::U224,
};

pub struct FileTable<'a, T: Io>(pub &'a T);

pub const CDT0: &str = "cdt0";

pub const ROOTS: &str = "roots";

pub const PARTS: &str = "parts";

fn path(t: Type, key: &U224) -> String {
    let s = key.to_base32();
    CDT0.to_owned()
        + "/"
        + [ROOTS, PARTS][t as usize]
        + "/"
        + &s[..2]
        + "/"
        + &s[2..4]
        + "/"
        + &s[4..]
}

impl<'a, T: Io> Forest for FileTable<'a, T> {
    fn has_block(&self, t: Type, key: &U224) -> bool {
        self.0.metadata(&path(t, key)).is_ok()
    }

    fn get_block(&self, t: Type, key: &U224) -> io::Result<Vec<u8>> {
        self.0.read(&path(t, key))
    }

    fn set_block(
        &mut self,
        t: Type,
        key: &U224,
        value: impl Iterator<Item = u8>,
    ) -> io::Result<()> {
        let x = value.collect::<Vec<_>>();
        let p = path(t, key);
        self.0.write_recursively(&p, &x)
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::forest::forest::Type;

    use super::path;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let k = [
            0x0ae63892, 0xc81cd1b0, 0x4f97a944, 0x891a80e6, 0x9205f2b7, 0xc9d3c292, 0x397b08b5,
        ];
        path(Type::Root, &k);
    }
}
