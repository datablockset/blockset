pub trait ArrayEx {
    type Item;
    fn new_vec(self) -> Vec<Self::Item>;
}

impl<T: Sized, const N: usize> ArrayEx for [T; N] {
    type Item = T;
    fn new_vec(self) -> Vec<Self::Item> {
        let mut result = Vec::with_capacity(N);
        for i in self {
            result.push(i);
        }
        result
    }
}
