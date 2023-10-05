pub const fn div_rem(a: usize, b: usize) -> (usize, usize) {
    (a / b, a % b)
}

#[cfg(test)]
mod test {
    use crate::div_rem::div_rem;

    #[test]
    fn test() {
        assert_eq!(div_rem(20, 3), (6, 2));
    }
}
