use std::io;

use crate::{u224::U224, from_u8x4};

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Main = 0,
    Parts = 1,
}

pub trait Table {
    fn has_block(&self, t: Type, key: &U224) -> bool;
    fn get_block(&self, t: Type, key: &U224) -> io::Result<Vec<u8>>;
    fn set_block(&mut self, t: Type, key: &U224, value: impl Iterator<Item = u8>) -> io::Result<()>;
    fn restore(&self, t: Type, k: &U224) -> io::Result<Vec<u8>> {
        let mut v = self.get_block(t, &k)?;
        let mut len = *v.first().unwrap() as usize;
        if len == 0x20 {
            v.remove(0);
            Ok(v)
        } else {
            let mut result = Vec::new();
            len += 1;
            let mut i = len;
            while i + 28 <= v.len() {
                let mut kn = U224::default();
                for ki in &mut kn {
                    let n = i + 4;
                    let slice = &v[i..n];
                    *ki = from_u8x4(slice.try_into().unwrap());
                    i = n;
                }
                result.extend(self.restore(Type::Parts, &kn)?);
            }
            result.extend(&v[1..len]);
            Ok(result)
        }
    }
}
