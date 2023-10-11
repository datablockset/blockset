use crate::{Io, u224::U224, storage::Storage, u256::{to_u224, U256}, digest::len};

struct FileStorage<'a, T: Io> {
    io: &'a mut T,
    data: Vec<u8>,
    nodes: Vec<Vec<U256>>,
}

impl<'a, T: Io> Storage for FileStorage<'a, T> {
    fn store(&mut self, digest: &U256, mut level: usize) {
        if level < 8 {
            if level == 0 {
                assert_eq!(digest[1], 0x08000000_00000000_00000000_00000000);
                self.data.push(digest[0] as u8);
            }
            return;
        }
        level -= 8;
        if level & 3 != 0 {
            return;
        }
        level >>= 2;
        self.nodes[level].push(*digest);
        if let Some(k) = to_u224(digest) {
            if level == 0 {
                assert!(self.data.len() >= 32);
                todo!("save data with key 'k'");
                self.data.clear();
            } else {
                todo!("save node with key 'k'");
                self.nodes[level - 1].clear();
            }
        } else {
            let len_bits = len(digest);
            assert_eq!(len_bits & 7, 0);
            assert_eq!(len_bits >> 3, self.data.len());
        }
    }

    fn end(&mut self, digest: &U256, level: usize) {
        todo!()
    }
}
