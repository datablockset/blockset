use crate::{
    digest::len,
    sha224::compress_one,
    storage::Storage,
    u224::U224,
    u256::{to_u224, U256},
    Io,
};

#[derive(Default)]
struct Level {
    nodes: Vec<U224>,
    last: U256,
}

struct Level4Storage<'a, T: Io> {
    io: &'a mut T,
    data: Vec<u8>,
    levels: Vec<Level>,
}

impl<'a, T: Io> Storage for Level4Storage<'a, T> {
    fn store(&mut self, digest: &U256, mut i: usize) {
        if i < 8 {
            if i == 0 {
                assert_eq!(digest[1], 0x08000000_00000000_00000000_00000000);
                self.data.push(digest[0] as u8);
            }
            return;
        }
        i -= 8;
        if i & 3 != 0 {
            return;
        }
        i >>= 2;
        if i >= self.levels.len() {
            self.levels.push(Level::default());
        }
        let level = &mut self.levels[i];
        if let Some(k) = to_u224(digest) {
            level.nodes.push(k);
            if i == 0 {
                assert!(self.data.len() >= 32);
                todo!("save data with key 'k'");
            } else {
                todo!("save node with key 'k'");
                self.levels[i - 1] = Level::default();
            }
            // always clear raw data storage after saving
            self.data.clear();
        } else {
            level.last = *digest;
            let len_bits = len(digest);
            assert_eq!(len_bits & 7, 0);
            assert_eq!(len_bits >> 3, self.data.len());
        }
    }

    fn end(&mut self, digest: &U224, level: usize) {
        if level == 0 {
            assert_eq!(*digest, compress_one(&[0, 0]));
            return;
        }
        if level <= 8 {}
    }
}
