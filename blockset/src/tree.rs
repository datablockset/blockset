use std::io;

use crate::{
    digest::to_digest, sha224::compress_one, storage::Storage, subtree::SubTree, u224::U224,
};

pub struct Tree<T: Storage> {
    state: Vec<SubTree>,
    storage: T,
}

impl<T: Storage> Tree<T> {
    pub fn new(storage: T) -> Self {
        Self {
            state: Vec::default(),
            storage,
        }
    }
    pub fn push(&mut self, c: u8) -> io::Result<()> {
        let mut i = 0;
        let mut last0 = to_digest(c);
        loop {
            let x = self.storage.store(&last0, i);
            x?;
            if let Some(sub_tree) = self.state.get_mut(i) {
                if let Some(last1) = sub_tree.push(&last0) {
                    last0 = last1;
                    i += 1;
                } else {
                    return Ok(());
                }
            } else {
                self.state.push(SubTree::new(&last0));
                return Ok(());
            }
        }
    }
    pub fn end(&mut self) -> io::Result<U224> {
        let mut last0 = [0, 0];
        for (i, sub_tree) in self.state.iter_mut().enumerate() {
            if last0 != [0, 0] {
                self.storage.store(&last0, i)?;
            }
            last0 = sub_tree.end(last0);
        }
        let key = compress_one(&last0);
        self.storage.end(&key, self.state.len())?;
        Ok(key)
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        digest::{merge, to_digest},
        sha224::compress_one,
        storage::Storage,
        u224::U224,
        u256::U256,
    };

    use super::Tree;

    #[derive(Default)]
    struct MemStorage(Vec<(U256, usize)>);

    impl Storage for MemStorage {
        fn store(&mut self, digest: &U256, index: usize) -> io::Result<()> {
            self.0.push((*digest, index));
            Ok(())
        }
        fn end(&mut self, _digest: &U224, _index: usize) -> io::Result<()> {
            Ok(())
        }
    }

    fn tree() -> Tree<MemStorage> {
        Tree::new(MemStorage::default())
    }

    pub fn tree_from_str(s: &str) -> (Vec<(U256, usize)>, U224) {
        let mut t = tree();
        for c in s.bytes() {
            t.push(c).unwrap();
        }
        let root = t.end().unwrap();
        (t.storage.0, root)
    }

    #[wasm_bindgen_test]
    #[test]
    fn empty_test() {
        let mut t = tree();
        assert_eq!(t.end().unwrap(), compress_one(&[0, 0]));
    }

    #[wasm_bindgen_test]
    #[test]
    fn hello_world_test() {
        //  48656c6c6f2c20776f726c6421
        // "H e l l o , _ w o r l d ! "
        let x = tree_from_str("Hello, world!");
        // println!("x: {:x?}", x);
        let e = [
            0x00000021_646c726f_77202c6f_6c6c6548,
            0x68000000_00000000_00000000_00000000,
        ];
        assert_eq!(x.1, compress_one(&e));
    }

    #[wasm_bindgen_test]
    #[test]
    fn content_dependent_hash_tree() {
        //  436f6e74656e742d446570656e64656e7420486173682054726565
        //  0       1       2       3       4       5       6
        // "C o n t e n t - D e p e n d e n t _ H a s h _ T r e e "
        let x = tree_from_str("Content-Dependent Hash Tree");
        let e: U256 = [
            0x6e65646e_65706544_2d746e65_746e6f43,
            0xD8000000_00656572_54206873_61482074,
        ];
        // println!("x: {:x?}", x);
        // println!("e: {:x?}", e);
        assert_eq!(x.1, compress_one(&e));
    }

    struct BrokenStorage();

    impl Storage for BrokenStorage {
        fn store(&mut self, _digest: &U256, _index: usize) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "BrokenStorage"))
        }
        fn end(&mut self, _digest: &U224, _index: usize) -> io::Result<()> {
            Err(io::Error::new(io::ErrorKind::Other, "BrokenStorage"))
        }
    }

    #[wasm_bindgen_test]
    #[test]
    fn fail_store_test() {
        let mut t = Tree::new(BrokenStorage());
        assert!(t.push(b'a').is_err());
    }

    #[wasm_bindgen_test]
    #[test]
    fn text_test() {
        // len: 1ab
        // 496d6167696e6520696e74657263657074696e67206d657373616765732066726f6d206578747261746572726573747269616c732e
        // 0       1       2       3       4       5       6       7       8       9       A       B       C       D
        // I m a g i n e _ i n t e r c e p t i n g _ m e s s a g e s _ f r o m _ e x t r a t e r r e s t r i a l s .
        //                                   |                                     |
        let v = "Imagine intercepting messages from extraterrestrials.";
        // - 0x24 B  = 0x120 b
        //   - 0x11
        //     - 0x6
        //       - "Im"
        //       - "ag"
        //       - "in"
        //     - 0xB
        //       - "e i"
        //       - "nt"
        //       - "er"
        //       - "ce"
        //       - "pt"
        //   - 0x13
        //     - 0x5
        //       - "in"
        //       - "g m"
        //     - 0x5
        //       - "es"
        //       - "sag"
        //     - 0x9
        //       - "es"
        //       - " f"
        //       - "rom e"
        // - 0x11
        //   - 0xA
        //     - "xtrat"
        //     - "er"
        //     - "res"
        //   - 0x7
        //     - "trial"
        //     - "s."

        // a = "Imagine intercept"
        let a: U256 = [
            //p e c r  e t n i  _ e n i  g a m I
            0x70656372_65746e69_20656e69_67616d49,
            //__                               t
            0x88000000_00000000_00000000_00000074,
        ];
        // b = "ing messages from e"
        let b: U256 = [
            //o r f  _ s e g a  s s e m  _ g n i
            0x6f7266_2073656761_7373656d_20676e69,
            //__                           e _ m
            0x98000000_00000000_00000000_0065206d,
        ];
        // c = "xtraterrestrials."
        let c: U256 = [
            //s l a i  r t s e  r r e t  a r t x
            0x736c6169_72747365_72726574_61727478,
            //__                               .
            0x88000000_00000000_00000000_0000002e,
        ];
        let x = tree_from_str(v);
        assert_eq!(x.1, compress_one(&merge(&merge(&a, &b), &c)));
        //
        let ciu = to_digest(b'I');
        let cm = to_digest(b'm');
        let ca = to_digest(b'a');
        let cg = to_digest(b'g');
        let ci = to_digest(b'i');
        let cn = to_digest(b'n');
        let cium = merge(&ciu, &cm);
        let cag = merge(&ca, &cg);
        let cin = merge(&ci, &cn);
        let ciumagin = merge(&merge(&cium, &cag), &cin);
        let ce = to_digest(b'e');
        let csp = to_digest(b' ');
        let ct = to_digest(b't');
        let cr = to_digest(b'r');
        let cc = to_digest(b'c');
        let cp = to_digest(b'p');
        let cespi = merge(&merge(&ce, &csp), &ci);
        let cnt = merge(&cn, &ct);
        let cer = merge(&ce, &cr);
        let cce = merge(&cc, &ce);
        let cpt = merge(&cp, &ct);
        let cespintercept = merge(&merge(&cespi, &cnt), &merge(&merge(&cer, &cce), &cpt));
        let cgspm = merge(&merge(&cg, &csp), &cm);
        // let cx = to_digest(b'x');
        let cs = to_digest(b's');
        let c = [
            (ciu, 0),
            (cm, 0),
            (cium, 1),
            (ca, 0),
            (cg, 0),
            (cag, 1),
            (ci, 0),
            (cn, 0),
            (cin, 1),
            (ciumagin, 2),
            (ce, 0),
            (csp, 0),
            (ci, 0),
            (cespi, 1),
            (cn, 0),
            (ct, 0),
            (cnt, 1),
            (ce, 0),
            (cr, 0),
            (cer, 1),
            (cc, 0),
            (ce, 0),
            (cce, 1),
            (cp, 0),
            (ct, 0),
            (cpt, 1),
            (cespintercept, 2),
            (merge(&ciumagin, &cespintercept), 3),
            (ci, 0),
            (cn, 0),
            (cin, 1),
            // "g m"
            (cg, 0),
            (csp, 0),
            (cm, 0),
            (cgspm, 1),
            (merge(&cin, &cgspm), 2),
            // "es"
            (ce, 0),
            (cs, 0),
            // "xtrat"
            //(cx, 0),
            //(ct, 0),
            //(cr, 0),
            //(ca, 0),
            //(ct, 0),
            //(merge(&merge(&cx, &ct), &merge(&cr, &merge(&ca, &ct))), 1)
        ];
        assert_eq!(x.0[..c.len()], c);
    }
}
