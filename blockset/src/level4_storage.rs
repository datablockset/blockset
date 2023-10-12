use std::{iter::once, mem::take};

use crate::{
    digest::len,
    sha224::compress_one,
    storage::Storage,
    table::Table,
    u224::U224,
    u256::{to_u224, U256},
    u32::to_u8x4,
};

#[derive(Default)]
struct Nodes {
    nodes: Vec<U224>,
    last: U256,
}

struct Levels {
    data: Vec<u8>,
    nodes: Vec<Nodes>,
}

impl Levels {
    fn store(&mut self, table: &mut impl Table, i: usize, k: &U224) {
        let data = take(&mut self.data);
        if i == 0 {
            assert!(data.len() > 0);
            table.set(k, once(0x20).chain(data));
        } else {
            let ref_level = &mut self.nodes[i - 1];
            let level = take(ref_level);
            {
                let len_bits = len(&level.last);
                assert_eq!(len_bits & 7, 0);
                assert_eq!(len_bits >> 3, data.len());
            }
            // we should have at least one node.
            assert_ne!(level.nodes.len(), 0);
            table.set(
                &to_u224(&level.last).unwrap(),
                once(data.len() as u8)
                    .chain(data)
                    .chain(level.nodes.into_iter().flatten().flat_map(to_u8x4)),
            );
            assert_eq!(ref_level.nodes.len(), 0);
            assert_eq!(ref_level.last, [0, 0]);
        }
    }
}

struct Level4Storage<P: Table, M: Table> {
    part_table: P,
    main_table: M,
    levels: Levels,
}

impl<P: Table, M: Table> Storage for Level4Storage<P, M> {
    fn store(&mut self, digest: &U256, mut i: usize) {
        if i < 8 {
            if i == 0 {
                assert_eq!(digest[1], 0x08000000_00000000_00000000_00000000);
                self.levels.data.push(digest[0] as u8);
            }
            return;
        }
        i -= 8;
        if i & 3 != 0 {
            return;
        }
        i >>= 2;
        if i >= self.levels.nodes.len() {
            self.levels.nodes.push(Nodes::default());
        }
        let level = &mut self.levels.nodes[i];
        if let Some(k) = to_u224(digest) {
            level.nodes.push(k);
            self.levels.store(&mut self.part_table, i, &k);
        } else {
            level.last = *digest;
            {
                let len_bits = len(digest);
                assert_eq!(len_bits & 7, 0);
                assert_eq!(len_bits >> 3, self.levels.data.len());
            }
        }
    }

    fn end(&mut self, k: &U224, mut i: usize) {
        if i == 0 {
            assert_eq!(*k, compress_one(&[0, 0]));
            return;
        }
        i = if i <= 8 { 0 } else { (i - 8) >> 2 };
        self.levels.store(&mut self.main_table, i, k);
    }
}
