pub const fn static_assert(v: bool) -> () {
    [0][!v as usize];
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::static_assert;

    #[test]
    fn test() {
        static_assert(true);
    }

    #[test]
    fn test_panic() {
        panic::catch_unwind(|| static_assert(false)).unwrap_err();
    }
}
