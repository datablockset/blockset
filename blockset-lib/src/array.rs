pub trait ArrayEx {
    type Item;
    fn move_to_vec(self) -> Vec<Self::Item>;
}

impl<T: Sized, const N: usize> ArrayEx for [T; N] {
    type Item = T;
    fn move_to_vec(self) -> Vec<Self::Item> {
        let mut result = Vec::with_capacity(N);
        for i in self {
            result.push(i);
        }
        result
    }
}
