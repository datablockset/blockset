use crate::{
    merge,
    u256::{less, U256},
};

// It should work faster than (a ^ b).leading_zeros().
pub const fn diff(&[a0, a1]: &U256, &[b0, b1]: &U256) -> u32 {
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
    fn new(&root: &U256, &last: &U256, height: u32) -> Self {
        Node { root, last, height }
    }
}

#[derive(Default)]
pub struct SubTree(Vec<Node>);

impl SubTree {
    pub fn push(&mut self, z: &U256) -> Option<U256> {
        let mut yz = 0;
        if let Some(mut y) = self.0.pop() {
            // z >= y.last
            if !less(z, &y.last) {
                let mut y = y.root;
                let mut root = *z;
                loop {
                    root = merge(&y, &root);
                    if let Some(x) = self.0.pop() {
                        y = x.root;
                    } else {
                        return Some(root);
                    }
                }
            }
            yz = diff(&y.last, z);
            loop {
                // we need `<=` instead of `<` to handle a case when `yz` and `y.height` are both zero.
                if y.height <= yz {
                    break;
                }
                let x = self.0.pop().unwrap();
                y = Node {
                    root: merge(&x.root, &y.root),
                    last: y.last,
                    height: x.height,
                };
            }
            self.0.push(y);
        };
        self.0.push(Node::new(z, z, yz));
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{merge, subtree::Node, to_digest};

    use super::{diff, SubTree};

    #[test]
    fn test() {
        assert_eq!(diff(&[0, 0], &[0, 0]), 256);
        assert_eq!(diff(&[0, 0], &[1, 0]), 255);
        assert_eq!(diff(&[0, 0], &[0, 1]), 127);
        assert_eq!(diff(&[0, 1], &[0, 1]), 256);
        assert_eq!(diff(&[1, 0], &[1, 0]), 256);
        assert_eq!(diff(&[0, 0], &[458, 1]), 127);
        assert_eq!(diff(&[0, 0], &[1, 0b1_1100_1010]), 119);
        assert_eq!(diff(&[0, 0b111], &[458, 0b100]), 126);
        assert_eq!(
            diff(&[0, 0], &[458, 0x80000000_00000000_00000000_00000000]),
            0
        );
    }

    #[test]
    fn subtree_test() {
        let a = to_digest(0b01);
        let b = to_digest(0b10);
        let c = to_digest(0b11);
        {
            let mut t = SubTree(Vec::default());
            assert_eq!(t.push(&a), None);
            assert_eq!(t.0, [Node::new(&a, &a, 0)]);
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
            assert_eq!(t.0, [Node::new(&c, &c, 0), Node::new(&b, &b, 255),]);
            assert_eq!(t.push(&a), None);
            let cb = merge(&c, &b);
            assert_eq!(t.0, [Node::new(&cb, &b, 0), Node::new(&a, &a, 254)],);
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
        assert_eq!(t.0, [Node::new(&ff, &ff, 0)]);
        //
        assert_eq!(t.push(&fe), None);
        assert_eq!(t.0, [Node::new(&ff, &ff, 0), Node::new(&fe, &fe, 255)]);
        //
        assert_eq!(t.push(&fd), None);
        let ff_fe = merge(&ff, &fe);
        assert_eq!(t.0, [Node::new(&ff_fe, &fe, 0), Node::new(&fd, &fd, 254)]);
        //
        assert_eq!(t.push(&fc), None);
        assert_eq!(
            t.0,
            [
                Node::new(&ff_fe, &fe, 0),
                Node::new(&fd, &fd, 254),
                Node::new(&fc, &fc, 255)
            ]
        );
        //
        assert_eq!(t.push(&fb), None);
        let ff_fc = merge(&ff_fe, &merge(&fd, &fc));
        assert_eq!(
            t.0,
            [
                Node::new(&&ff_fc, &fc, 0),
                Node::new(&fb, &fb, 253),
            ]
        );
        //
        assert_eq!(t.push(&fa), None);
        assert_eq!(t.0,
            [
                Node::new(&&ff_fc, &fc, 0),
                Node::new(&fb, &fb, 253),
                Node::new(&fa, &fa, 255),
            ]
        );
        //
        assert_eq!(t.push(&f9), None);
        let fb_fa = merge(&fb, &fa);
        assert_eq!(t.0,
            [
                Node::new(&&ff_fc, &fc, 0),
                Node::new(&fb_fa, &fa, 253),
                Node::new(&f9, &f9, 254),
            ]
        );
        //
        assert_eq!(t.push(&f8), None);
        assert_eq!(t.0,
            [
                Node::new(&&ff_fc, &fc, 0),
                Node::new(&fb_fa, &fa, 253),
                Node::new(&f9, &f9, 254),
                Node::new(&f8, &f8, 255),
            ]
        );
        //
        assert_eq!(t.push(&f7), None);
        let ff_f8 = merge(&ff_fc, &merge(&fb_fa, &merge(&f9, &f8)));
        assert_eq!(t.0,
            [
                Node::new(&ff_f8, &f8, 0),
                Node::new(&f7, &f7, 252),
            ]
        );
        //
        assert_eq!(t.push(&f6), None);
        assert_eq!(t.0,
            [
                Node::new(&ff_f8, &f8, 0),
                Node::new(&f7, &f7, 252),
                Node::new(&f6, &f6, 255),
            ]
        );
        //
        assert_eq!(t.push(&f5), None);
        assert_eq!(t.0,
            [
                Node::new(&ff_f8, &f8, 0),
                Node::new(&merge(&f7, &f6), &f6, 252),
                Node::new(&f5, &f5, 254),
            ]
        );
    }
}
