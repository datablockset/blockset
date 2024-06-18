use crate::uint::u256x::{self, U256};

const P: U256 = [
    0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
    0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
];

const fn is_valid(key: U256) -> bool {
    u256x::less(&key, &P)
}

const fn is_valid_private_key(key: U256) -> bool {
    u256x::less(&u256x::ZERO, &key) && is_valid(key)
}

#[derive(Debug, PartialEq)]
struct Scalar(U256);

impl Scalar {
    #[inline(always)]
    const fn new(key: U256) -> Self {
        assert!(is_valid(key));
        Self(key)
    }
    #[inline(always)]
    const fn eq(self, b: Self) -> bool {
        u256x::eq(&self.0, &b.0)
    }
    const fn add(self, b: Self) -> Self {
        let (mut result, o) = u256x::oadd(self.0, b.0);
        if o || !u256x::less(&result, &P) {
            result = u256x::wsub(result, P)
        }
        Scalar(result)
    }
}

const B: U256 = u256x::from_u128(7);

struct Compressed {
    parity: bool,
    x: U256,
}

struct Uncompressed {
    x: U256,
    y: U256,
}

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::secp256k1::{is_valid_private_key, Scalar};

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        //
        assert!(!is_valid_private_key([0, 0]));
        assert!(is_valid_private_key([1, 0]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0
        ]));
        //
        assert!(is_valid_private_key([0, 1]));
        assert!(is_valid_private_key([1, 1]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            1
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            1
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            1
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFE
        ]));
        //
        assert!(is_valid_private_key([
            0,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(is_valid_private_key([
            1,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(!is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2F,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
        assert!(!is_valid_private_key([
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF,
            0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
        ]));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_add() {
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .add(Scalar::new([1, 0])),
            Scalar::new([0, 0])
        );
        assert_eq!(
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
            .add(Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2E,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])),
            Scalar::new([
                0xFFFFFFFF_FFFFFFFF_FFFFFFFE_FFFFFC2D,
                0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF
            ])
        );
    }
}
