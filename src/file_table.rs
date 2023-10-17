use std::io;

use crate::{
    base32::ToBase32,
    table::{Table, Type},
    u224::U224,
    Io,
};

pub struct FileTable<'a, T: Io>(pub &'a T);

pub const DIR: &str = "cdt0";

fn path(t: Type, key: &U224) -> String {
    let s = key.to_base32();
    DIR.to_owned()
        + "/"
        + ["roots", "parts"][t as usize]
        + "/"
        + &s[..2]
        + "/"
        + &s[2..4]
        + "/"
        + &s[4..]
}

impl<'a, T: Io> Table for FileTable<'a, T> {
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

    use crate::table::Type;

    use super::path;

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let k = [
            0x0ae63892, 0xc81cd1b0, 0x4f97a944, 0x891a80e6, 0x9205f2b7, 0xc9d3c292, 0x397b08b5,
        ];
        path(Type::Main, &k);
    }
}
