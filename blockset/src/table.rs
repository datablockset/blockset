use std::io::{self, Write};

use crate::{sha224::compress_one, u224::U224, u32::from_u8x4};

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Main = 0,
    Parts = 1,
}

const EMPTY: U224 = compress_one(&[0, 0]);

pub trait Table {
    fn has_block(&self, t: Type, key: &U224) -> bool;
    fn get_block(&self, t: Type, key: &U224) -> io::Result<Vec<u8>>;
    fn set_block(&mut self, t: Type, key: &U224, value: impl Iterator<Item = u8>)
        -> io::Result<()>;
    fn check_set_block(
        &mut self,
        t: Type,
        key: &U224,
        value: impl Iterator<Item = u8>,
    ) -> io::Result<()> {
        if !self.has_block(t, key) {
            self.set_block(t, key, value)?;
        }
        Ok(())
    }
    fn restore(&self, mut t: Type, k: &U224, w: &mut impl Write) -> io::Result<()> {
        if *k == EMPTY {
            return Ok(());
        }
        let mut tail = Vec::default();
        let mut keys = [*k].to_vec();
        while let Some(key) = keys.pop() {
            let v = self.get_block(t, &key)?;
            let mut len = *v.first().unwrap() as usize;
            if len == 0x20 {
                w.write_all(&v[1..])?;
            } else {
                len += 1;
                if len > 1 {
                    assert!(tail.is_empty());
                    tail = v[1..len].to_vec();
                }
                let mut i = v.len();
                while len + 28 <= i {
                    let mut kn = U224::default();
                    i -= 28;
                    let mut j = i;
                    for ki in &mut kn {
                        let n = j + 4;
                        let slice = &v[j..n];
                        *ki = from_u8x4(slice.try_into().unwrap());
                        j = n;
                    }
                    keys.push(kn);
                }
            }
            t = Type::Parts;
        }
        w.write_all(&tail)
    }
}
