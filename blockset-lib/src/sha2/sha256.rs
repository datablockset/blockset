use crate::uint::u256x::U256;

pub const SHA256: U256 = [
    0xa54ff53a_3c6ef372_bb67ae85_6a09e667,
    0x5be0cd19_1f83d9ab_9b05688c_510e527f,
];

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::sha2::{compress::compress, sha256::SHA256};

    #[wasm_bindgen_test]
    #[test]
    fn runtime_test() {
        let x = compress(SHA256, [[0x8000_0000, 0], [0, 0]]);
        assert_eq!(
            x,
            [
                0x996fb924_9afbf4c8_98fc1c14_e3b0c442,
                0x7852b855_a495991b_649b934c_27ae41e4,
            ]
        );
    }
}
