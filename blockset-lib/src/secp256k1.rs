use crate::uint::u256x::{self, U256};

const P: U256 = [0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF];

const B: U256 = u256x::from_u128(7);

const fn is_valid_private_key(key: U256) -> bool {
    u256x::less(&u256x::ZERO, &key) && u256x::less(&key, &P)
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::secp256k1::is_valid_private_key;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        //
        assert!(!is_valid_private_key([0, 0]));
        assert!(is_valid_private_key([1, 0]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E, 0]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 0]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF, 0]));
        //
        assert!(is_valid_private_key([0, 1]));
        assert!(is_valid_private_key([1, 1]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E, 1]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 1]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF, 1]));
        //
        assert!(is_valid_private_key([0, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E]));
        assert!(is_valid_private_key([1, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E]));
        //
        assert!(is_valid_private_key([0, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F]));
        assert!(is_valid_private_key([1, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF, 0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F]));
        //
        assert!(is_valid_private_key([0, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE]));
        assert!(is_valid_private_key([1, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE]));
        //
        assert!(is_valid_private_key([0, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF]));
        assert!(is_valid_private_key([1, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF]));
        assert!(is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF]));
        assert!(!is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF]));
        assert!(!is_valid_private_key([0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF, 0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF]));
    }
}