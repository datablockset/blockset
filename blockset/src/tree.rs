use crate::{to_digest, u256::U256, SubTree};

struct Tree(Vec<SubTree>);

impl Tree {
    fn push(&mut self, c: u8) {
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
    fn end(&mut self) -> U256 {
        let mut last0 = [0, 0];
        for sub_tree in self.0.iter_mut() {
            last0 = sub_tree.end(last0);
        }
        last0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {}
}
