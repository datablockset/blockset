use std::{io, iter::once, mem::take};

use super::{Forest, Type};

use crate::{
    cdt::{
        node_id::{len, root},
        tree_add::TreeAdd,
    },
    uint::{
        u224::U224,
        u256::{to_u224, U256},
        u32::to_u8x4,
    },
};

#[derive(Default)]
struct Nodes {
    nodes: Vec<U224>,
    last: U256,
}

#[derive(Default)]
struct ForestTreeAdd {
    data: Vec<u8>,
    nodes: Vec<Nodes>,
}

const DATA_LEVEL: usize = 8;
const SKIP_LEVEL: usize = 4;

impl ForestTreeAdd {
    fn store(&mut self, forest: &mut impl Forest, t: Type, i: usize, k: &U224) -> io::Result<u64> {
        let data = take(&mut self.data);
        let data_len = data.len();
        let r = if i == 0 {
            assert!(!data.is_empty());
            forest.check_set_block(t, k, once(0x20).chain(data))?
        } else {
            let ref_level = &mut self.nodes[i - 1];
            let level = take(ref_level);
            {
                let len_bits = len(&level.last);
                assert_eq!(len_bits & 7, 0);
                assert_eq!(len_bits >> 3, data_len);
            }
            // We should have at least one node because `k`
            // can't be formed from `last` only.
            // If the first node is equal to the original `k` then
            // we don't need to store it.
            if level.nodes.first().unwrap() == k {
                // only one node can produce the same digest.
                assert_eq!(level.nodes.len(), 1);
                // no additional data should be present.
                assert_eq!(level.last, [0, 0]);
                return Ok(0); // already stored
            }
            forest.check_set_block(
                t,
                k,
                once(data_len as u8)
                    .chain(data)
                    .chain(level.nodes.into_iter().flatten().flat_map(to_u8x4)),
            )?
        };
        Ok(if r { data_len as u64 } else { 0 })
    }
}

pub struct LevelStorage<T: Forest> {
    forest: T,
    levels: ForestTreeAdd,
}

impl<T: Forest> LevelStorage<T> {
    pub fn new(forest: T) -> Self {
        Self {
            forest,
            levels: Default::default(),
        }
    }
}

impl<T: Forest> TreeAdd for LevelStorage<T> {
    fn push(&mut self, digest: &U256, mut i: usize) -> io::Result<u64> {
        if i < DATA_LEVEL {
            if i == 0 {
                assert_eq!(digest[1], 0x08000000_00000000_00000000_00000000);
                self.levels.data.push(digest[0] as u8);
            }
            return Ok(0);
        }
        i -= DATA_LEVEL;
        if i % SKIP_LEVEL != 0 {
            return Ok(0);
        }
        i /= SKIP_LEVEL;
        if i >= self.levels.nodes.len() {
            self.levels.nodes.push(Nodes::default());
        }
        let level = &mut self.levels.nodes[i];
        if let Some(k) = to_u224(digest) {
            level.nodes.push(k);
            self.levels.store(&mut self.forest, Type::Child, i, &k)
        } else {
            level.last = *digest;
            {
                let len_bits = len(digest);
                assert_eq!(len_bits & 7, 0);
                assert_eq!(len_bits >> 3, self.levels.data.len());
            }
            Ok(0)
        }
    }

    fn end(&mut self, k: &U224, mut i: usize) -> io::Result<u64> {
        if i == 0 {
            assert_eq!(*k, root(&[0, 0]));
            return Ok(0);
        }
        i = if i <= DATA_LEVEL {
            0
        } else {
            (i - DATA_LEVEL + SKIP_LEVEL - 1) / SKIP_LEVEL
        };
        self.levels.store(&mut self.forest, Type::Root, i, k)
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        cdt::{main_tree::MainTreeAdd, tree_add::TreeAdd},
        forest::{mem::Mem, Forest, Type},
        uint::u224::U224,
    };

    use super::LevelStorage;

    fn tree_from_str<T: TreeAdd>(tree: &mut MainTreeAdd<T>, s: &str) -> U224 {
        for c in s.bytes() {
            tree.push(c).unwrap();
        }
        tree.end().unwrap().0
    }

    fn add(table: &mut Mem, c: &str) -> U224 {
        let mut tree = MainTreeAdd::new(LevelStorage::new(table));
        tree_from_str(&mut tree, c)
    }

    fn small(c: &str) {
        let mut table = Mem::default();
        let k = add(&mut table, c);
        let v = (&mut table).get_block(Type::Root, &k).unwrap();
        assert_eq!(v, (" ".to_owned() + c).as_bytes());
    }

    fn big(c: &str) {
        let table = &mut Mem::default();
        let k = add(table, c);
        let mut v = Vec::default();
        let mut cursor = Cursor::new(&mut v);
        table
            .restore(
                Type::Root,
                &k,
                &mut cursor,
                &mut Cursor::<Vec<_>>::default(),
            )
            .unwrap();
        assert_eq!(v, c.as_bytes());
    }

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        small("Hello, world!");
        small("Content-Dependent Hash Tree");
        small(
            r#"Imagine intercepting messages from extraterrestrials.
            We don’t know their language, but we assume that they use a
            sequential language unless they are from the Arrival film.
            The messages manifest as a sequence of numbers. We know that each
            number can be a finite number between 0 and N-1. How can we
            structure the stream without linguistic reference points? How do we
            identify repetitive segments?"#,
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn test_big() {
        big(r#"There are a lot of articles, videos, and blog posts about
            functional programming using different programming languages,
            including JavaScript.

            Usually, the main topic of these articles is
            how to use various functional programming paradigms, such as
            first-class functions, immutable objects, and currying.

            Nevertheless, the primary value of purely functional programming
            languages is an absence of side effects. Partial applications of
            different functional paradigms in impure languages, such as
            JavaScript, may reduce the number of side effects but don’t
            guarantee their complete elimination.

            Side effects reduce scalability and the ability to replace
            components and platforms. So, it is preferable to reduce the number
            of side effects to a bare minimum.

            There are dozens of purely functional programming languages. Some of
            them are pretty successful in the software development industry —
            for example, Haskell, Elm, and PureScript. However, the most popular
            programming language is JavaScript, and it is not purely functional.

            The main reason to use JavaScript, besides its popularity, is that
            almost any web browser can run it. Also, one of the most popular
            data interchange and file formats is JSON, a subset of JavaScript.
            Because of this JSON/JavaScript relation, serialization in
            JavaScript is more straightforward than in other programming
            languages. In my experience, object-oriented programming languages
            usually have the biggest challenges in serialization.

            Any working program has side effects such as input/output, functions
            that return the current time, or random numbers.

            But it is possible to write a big part of a program without using
            impure functions. An impure function can be rewritten as a pure
            function.

            For example:

            Pure functions are much more flexible. A developer may use the
            pureAddAndPrint function with either pure or impure arguments, such
            as console.log. Some platforms may not have console.log, and in that
            case, a developer could provide a replacement for it.

            Another use case is unit testing, and a developer may create a mock
            function and pass it as an argument.

            Currying

            You may notice function declarations in this article use currying.
            In most purely functional programming languages, a function can
            accept only one argument, and currying is a way to provide multiple
            arguments to a function."#);
    }
}
