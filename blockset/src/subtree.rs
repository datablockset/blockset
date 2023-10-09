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

pub struct SubTree(Vec<Node>);

impl SubTree {
    pub fn push(&mut self, z: &U256) -> Option<U256> {
        let mut yz = 0;
        if let Some(mut y) = self.0.pop() {
            // z >= y.last
            if !less(z, &y.last) {
                let mut root = merge(&y.root, z);
                while let Some(y) = self.0.pop() {
                    root = merge(&y.root, &root);
                }
                return Some(root);
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
        self.0.push(Node {
            root: *z,
            last: *z,
            height: yz,
        });
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
            assert_eq!(
                t.0,
                [Node {
                    root: a,
                    last: a,
                    height: 0
                }]
            );
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
            assert_eq!(
                t.0,
                [
                    Node {
                        root: c,
                        last: c,
                        height: 0
                    },
                    Node {
                        root: b,
                        last: b,
                        height: 255
                    }
                ]
            );
            assert_eq!(t.push(&a), None);
            assert_eq!(
                t.0,
                [
                    Node {
                        root: merge(&c, &b),
                        last: b,
                        height: 0
                    },
                    Node {
                        root: a,
                        last: a,
                        height: 254
                    }
                ],
            );
        }
    }
}
