use crate::{digest::to_digest, subtree::SubTree, u256::U256};

#[derive(Default)]
pub struct Tree(Vec<SubTree>);

impl Tree {
    pub fn push(&mut self, c: u8) {
        let mut i = 0;
        let mut last0 = to_digest(c);
        loop {
            if let Some(sub_tree) = self.0.get_mut(i) {
                if let Some(last1) = sub_tree.push(&last0) {
                    last0 = last1;
                    i += 1;
                } else {
                    return;
                }
            } else {
                self.0.push(SubTree::new(&last0));
                return;
            }
        }
    }
    pub fn end(&mut self) -> U256 {
        let mut last0 = [0, 0];
        for sub_tree in self.0.iter_mut() {
            last0 = sub_tree.end(last0);
        }
        last0
    }
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{digest::merge, u256::U256};

    use super::Tree;

    pub fn tree_from_str(s: &str) -> U256 {
        let mut t = Tree::default();
        for c in s.bytes() {
            t.push(c);
        }
        t.end()
    }

    #[wasm_bindgen_test]
    #[test]
    fn empty_test() {
        let mut t = Tree::default();
        assert_eq!(t.end(), [0, 0]);
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
        assert_eq!(x, e);
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
        println!("x: {:x?}", x);
        println!("e: {:x?}", e);
        assert_eq!(x, e);
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
        // - 0x24 (120)
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
        assert_eq!(x, merge(&merge(&a, &b), &c));
    }
}
