use std::{iter::once, mem::take, io};

use crate::{
    digest::len,
    sha224::compress_one,
    storage::Storage,
    table::{Table, Type},
    u224::U224,
    u256::{to_u224, U256},
    u32::to_u8x4,
};

#[derive(Default)]
struct Nodes {
    nodes: Vec<U224>,
    last: U256,
}

#[derive(Default)]
struct Levels {
    data: Vec<u8>,
    nodes: Vec<Nodes>,
}

const DATA_LEVEL: usize = 8;
const SKIP_LEVEL: usize = 4;

impl Levels {
    fn store(&mut self, table: &mut impl Table, t: Type, i: usize, k: &U224) -> io::Result<()> {
        let data = take(&mut self.data);
        if i == 0 {
            assert!(!data.is_empty());
            table.set_block(t, k, once(0x20).chain(data))?;
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
            table.set_block(
                t,
                k,
                once(data.len() as u8)
                    .chain(data)
                    .chain(level.nodes.into_iter().flatten().flat_map(to_u8x4)),
            )?;
            assert_eq!(ref_level.nodes.len(), 0);
            assert_eq!(ref_level.last, [0, 0]);
        }
        Ok(())
    }
}

pub struct LevelStorage<'a, T: Table> {
    table: &'a mut T,
    levels: Levels,
}

impl<'a, T: Table> LevelStorage<'a, T> {
    pub fn new(table: &'a mut T) -> Self {
        Self {
            table,
            levels: Default::default(),
        }
    }
}

impl<'a, T: Table> Storage for LevelStorage<'a, T> {
    fn store(&mut self, digest: &U256, mut i: usize) -> io::Result<()> {
        if i < DATA_LEVEL {
            if i == 0 {
                assert_eq!(digest[1], 0x08000000_00000000_00000000_00000000);
                self.levels.data.push(digest[0] as u8);
            }
            return Ok(());
        }
        i -= DATA_LEVEL;
        if i % SKIP_LEVEL != 0 {
            return Ok(());
        }
        i /= SKIP_LEVEL;
        if i >= self.levels.nodes.len() {
            self.levels.nodes.push(Nodes::default());
        }
        let level = &mut self.levels.nodes[i];
        if let Some(k) = to_u224(digest) {
            level.nodes.push(k);
            self.levels.store(self.table, Type::Parts, i, &k)?;
        } else {
            level.last = *digest;
            {
                let len_bits = len(digest);
                assert_eq!(len_bits & 7, 0);
                assert_eq!(len_bits >> 3, self.levels.data.len());
            }
        }
        Ok(())
    }

    fn end(&mut self, k: &U224, mut i: usize) -> io::Result<()> {
        if i == 0 {
            assert_eq!(*k, compress_one(&[0, 0]));
            return Ok(());
        }
        i = if i <= DATA_LEVEL {
            0
        } else {
            (i - DATA_LEVEL + SKIP_LEVEL - 1) / SKIP_LEVEL
        };
        self.levels.store(self.table, Type::Main, i, k)
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        mem_table::MemTable,
        storage::Storage,
        table::{Table, Type},
        tree::Tree,
        u224::U224,
        u32::from_u8x4,
    };

    use super::LevelStorage;

    fn tree_from_str<T: Storage>(tree: &mut Tree<T>, s: &str) -> U224 {
        for c in s.bytes() {
            tree.push(c).unwrap();
        }
        tree.end().unwrap()
    }

    fn add(table: &mut MemTable, c: &str) -> U224 {
        let mut tree = Tree::new(LevelStorage::new(table));
        tree_from_str(&mut tree, c)
    }

    fn small(c: &str) {
        let mut table = MemTable::default();
        let k = add(&mut table, c);
        let v = table.get_block(Type::Main,&k).unwrap();
        assert_eq!(v, (" ".to_owned() + c).as_bytes());
    }

    fn restore<T: Table>(table: &T, t: Type, k: &U224) -> io::Result<Vec<u8>> {
        let mut v = table.get_block(t, &k)?;
        let mut len = *v.first().unwrap() as usize;
        if len == 0x20 {
            v.remove(0);
            Ok(v)
        } else {
            let mut result = Vec::new();
            len += 1;
            let mut i = len;
            while i + 28 <= v.len() {
                let mut kn = U224::default();
                for ki in &mut kn {
                    let n = i + 4;
                    let slice = &v[i..n];
                    *ki = from_u8x4(slice.try_into().unwrap());
                    i = n;
                }
                result.extend(restore(table, Type::Parts, &kn)?);
            }
            result.extend(&v[1..len]);
            Ok(result)
        }
    }

    fn big(c: &str) {
        let mut table = MemTable::default();
        let k = add(&mut table, c);
        let v = restore(&table, Type::Main, &k).unwrap();
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