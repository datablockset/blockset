mod point;
mod scalar;

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        elliptic_curve::order::{verify, Order},
        secp256k1::scalar::Secp256k1P,
    };

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let f = |p, h| {
            // Alice
            let private_key = Order::<Secp256k1P>::new(p);
            let public_key = private_key.public_key();
            let hash = Order::new(h);
            let signature = private_key.sign(hash);
            // Bob
            let result = verify(public_key, hash, signature);
            assert!(result);
            // Enemy
            let w_private_key = private_key.add(Order::_1);
            let w_signature = w_private_key.sign(hash);
            // Bob
            let w_result = verify(public_key, hash, w_signature);
            assert!(!w_result);
        };
        f(
            [
                1234567890_1234567890_1234567890_123456789,
                234567890_1234567890_1234567890_1234567890,
            ],
            [
                34567890_1234567890_1234567890_1234567890,
                4567890_1234567890_1234567890_1234567890_1,
            ],
        );
        f(
            [
                7890_1234567890_1234567890_1234567890_1234,
                890_1234567890_1234567890_1234567890_12345,
            ],
            [
                90_1234567890_1234567890_1234567890_123456,
                1234567890_1234567890_1234567890_123456790,
            ],
        );
        f(
            [
                1111111111_2222222222_3333333333_444444444,
                4444444444_5555555555_6666666666_77777777,
            ],
            [
                8888888888_9999999999_0000000000_11111111,
                2222222222_3333333333_4444444444_555555555,
            ],
        );
        f([u128::MAX, u128::MAX], [u128::MAX, u128::MAX]);
    }
}
