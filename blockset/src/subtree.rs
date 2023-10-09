use crate::{
    merge,
    u256::{less, U256},
};

// It should work faster than (a ^ b).leading_zeros().
pub const fn height(&[a0, a1]: &U256, &[b0, b1]: &U256) -> u32 {
    let mut result = 0;
    let mut v = a1 ^ b1;
    if v == 0 {
        v = a0 ^ b0;
        result += 128;
    }
    result + v.leading_zeros()
}

#[derive(PartialEq, Debug)]
struct Node {
    root: U256,
    last: U256,
    height: u32,
}

impl Node {
    fn new2(last: &U256, height: u32) -> Self {
        Self::new3(last, last, height)
    }
    fn new3(&root: &U256, &last: &U256, height: u32) -> Self {
        Node { root, last, height }
    }
}

#[derive(Default)]
pub struct SubTree(Vec<Node>);

impl SubTree {
    pub fn new(last: &U256) -> Self {
        let mut result = Vec::default();
        result.push(Node::new2(last, 0));
        Self(result)
    }
    pub fn push(&mut self, last0: &U256) -> Option<U256> {
        let mut height10 = 0;
        if let Some(mut last1) = self.0.pop() {
            // last0 >= last1.last
            if !less(last0, &last1.last) {
                return Some(self.end(merge(&last1.root, last0)));
            }
            height10 = height(&last1.last, last0);
            loop {
                // we need `<=` instead of `<` to handle a case when `height10` and `last1.height` are both zero.
                if last1.height <= height10 {
                    break;
                }
                let last2 = self.0.pop().unwrap();
                last1 = Node {
                    root: merge(&last2.root, &last1.root),
                    last: last1.last,
                    height: last2.height,
                };
            }
            self.0.push(last1);
        };
        self.0.push(Node::new2(last0, height10));
        None
    }
    pub fn end(&mut self, mut last0: U256) -> U256 {
        while let Some(last1) = self.0.pop() {
            last0 = merge(&last1.root, &last0);
        }
        last0
    }
}

#[cfg(test)]
mod test {
    use crate::{merge, subtree::Node, to_digest};

    use super::{height, SubTree};

    #[test]
    fn test() {
        assert_eq!(height(&[0, 0], &[0, 0]), 256);
        assert_eq!(height(&[0, 0], &[1, 0]), 255);
        assert_eq!(height(&[0, 0], &[0, 1]), 127);
        assert_eq!(height(&[0, 1], &[0, 1]), 256);
        assert_eq!(height(&[1, 0], &[1, 0]), 256);
        assert_eq!(height(&[0, 0], &[458, 1]), 127);
        assert_eq!(height(&[0, 0], &[1, 0b1_1100_1010]), 119);
        assert_eq!(height(&[0, 0b111], &[458, 0b100]), 126);
        assert_eq!(
            height(&[0, 0], &[458, 0x80000000_00000000_00000000_00000000]),
            0
        );
    }

    #[test]
    fn subtree_test() {
        let a = to_digest(0b01);
        let b = to_digest(0b10);
        let c = to_digest(0b11);
        {
            let mut t = SubTree::new(&a);
            // assert_eq!(t.push(&a), None);
            assert_eq!(t.0, [Node::new2(&a, 0)]);
            assert_eq!(t.push(&b), Some(merge(&a, &b)));
            assert!(t.0.is_empty());
        }
        {
            let mut t = SubTree(Vec::default());
            assert_eq!(t.push(&c), None);
            assert_eq!(
                t.0,
                [Node {
                    root: c,
                    last: c,
                    height: 0
                }]
            );
            assert_eq!(t.push(&b), None);
            assert_eq!(t.0, [Node::new2(&c, 0), Node::new2(&b, 255),]);
            assert_eq!(t.push(&a), None);
            let cb = merge(&c, &b);
            assert_eq!(t.0, [Node::new3(&cb, &b, 0), Node::new2(&a, 254)],);
            let r = t.push(&a);
            assert_eq!(r, Some(merge(&cb, &merge(&a, &a))));
        }
    }

    #[test]
    fn subtree2_test() {
        let ff = to_digest(0b1111_1111); // 000
        let fe = to_digest(0b1111_1110); // 255 000
        let fd = to_digest(0b1111_1101); // 254
        let fc = to_digest(0b1111_1100); // 255 254 000
        let fb = to_digest(0b1111_1011); // 253
        let fa = to_digest(0b1111_1010); // 255 253
        let f9 = to_digest(0b1111_1001); // 254
        let f8 = to_digest(0b1111_1000); // 255 254 253 000
        let f7 = to_digest(0b1111_0111); // 252
        let f6 = to_digest(0b1111_0110); // 255 252
        let f5 = to_digest(0b1111_0100); // 254
        let mut t = SubTree::default();
        assert_eq!(t.push(&ff), None);
        assert_eq!(t.0, [Node::new2(&ff, 0)]);
        //
        assert_eq!(t.push(&fe), None);
        assert_eq!(t.0, [Node::new2(&ff, 0), Node::new2(&fe, 255)]);
        //
        assert_eq!(t.push(&fd), None);
        let ff_fe = merge(&ff, &fe);
        assert_eq!(t.0, [Node::new3(&ff_fe, &fe, 0), Node::new2(&fd, 254)]);
        //
        assert_eq!(t.push(&fc), None);
        assert_eq!(
            t.0,
            [
                Node::new3(&ff_fe, &fe, 0),
                Node::new2(&fd, 254),
                Node::new2(&fc, 255)
            ]
        );
        //
        assert_eq!(t.push(&fb), None);
        let ff_fc = merge(&ff_fe, &merge(&fd, &fc));
        assert_eq!(t.0, [Node::new3(&&ff_fc, &fc, 0), Node::new2(&fb, 253),]);
        //
        assert_eq!(t.push(&fa), None);
        assert_eq!(
            t.0,
            [
                Node::new3(&&ff_fc, &fc, 0),
                Node::new2(&fb, 253),
                Node::new2(&fa, 255),
            ]
        );
        //
        assert_eq!(t.push(&f9), None);
        let fb_fa = merge(&fb, &fa);
        assert_eq!(
            t.0,
            [
                Node::new3(&&ff_fc, &fc, 0),
                Node::new3(&fb_fa, &fa, 253),
                Node::new2(&f9, 254),
            ]
        );
        //
        assert_eq!(t.push(&f8), None);
        assert_eq!(
            t.0,
            [
                Node::new3(&&ff_fc, &fc, 0),
                Node::new3(&fb_fa, &fa, 253),
                Node::new2(&f9, 254),
                Node::new2(&f8, 255),
            ]
        );
        //
        assert_eq!(t.push(&f7), None);
        let ff_f8 = merge(&ff_fc, &merge(&fb_fa, &merge(&f9, &f8)));
        assert_eq!(t.0, [Node::new3(&ff_f8, &f8, 0), Node::new2(&f7, 252),]);
        //
        assert_eq!(t.push(&f6), None);
        assert_eq!(
            t.0,
            [
                Node::new3(&ff_f8, &f8, 0),
                Node::new2(&f7, 252),
                Node::new2(&f6, 255),
            ]
        );
        //
        assert_eq!(t.push(&f5), None);
        assert_eq!(
            t.0,
            [
                Node::new3(&ff_f8, &f8, 0),
                Node::new3(&merge(&f7, &f6), &f6, 252),
                Node::new2(&f5, 254),
            ]
        );
    }

    #[test]
    fn hi_test() {
        let a = to_digest(b'a');
        let b = to_digest(b'b');
        let ab = {
            let mut t = SubTree(Vec::default());
            assert_eq!(t.push(&a), None);
            assert_eq!(t.0, [Node::new2(&a, 0)]);
            let ab = t.push(&b);
            assert_eq!(ab, Some(merge(&a, &b)));
            assert!(t.0.is_empty());
            ab
        }
        .unwrap();
        let baa = {
            let mut t = SubTree(Vec::default());
            assert_eq!(t.push(&b), None);
            assert_eq!(t.0, [Node::new2(&b, 0)]);
            assert_eq!(t.push(&a), None);
            assert_eq!(t.0, [Node::new2(&b, 0), Node::new2(&a, 254)]);
            let baa = t.push(&a);
            assert_eq!(baa, Some(merge(&b, &merge(&a, &a))));
            assert!(t.0.is_empty());
            baa
        }
        .unwrap();
        {
            let mut t = SubTree(Vec::default());
            assert_eq!(t.push(&ab), None);
            assert_eq!(t.0, [Node::new2(&ab, 0)]);
            let r = t.push(&baa);
            assert_eq!(r, Some(merge(&ab, &baa)));
        }
    }
}
