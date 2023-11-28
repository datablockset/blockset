use crate::uint::u256::U256;

pub const SHA224: U256 = [
    0xf70e5939_3070dd17_367cd507_c1059ed8,
    0xbefa4fa4_64f98fa7_68581511_ffc00b31,
];

#[cfg(test)]
mod test {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{sha2::compress::compress, uint::u256::U256};

    use super::SHA224;

    const A: U256 = compress(SHA224, [[0x8000_0000, 0], [0, 0]]);

    const _: () = assert!({
        let x = A;
        x[0] == 0x288234c4_476102bb_2a3a2bc9_d14a028c
            && (x[1] & ((1 << 96) - 1)) == 0xc5b3e42f_828ea62a_15a2b01f
    });

    #[wasm_bindgen_test]
    #[test]
    fn test() {
        let mut x = A;
        x[1] |= 0xFFFF_FFFF << 96;
        assert_eq!(
            x,
            [
                0x288234c4_476102bb_2a3a2bc9_d14a028c,
                0xFFFFFFFF_c5b3e42f_828ea62a_15a2b01f,
            ]
        );
    }

    #[wasm_bindgen_test]
    #[test]
    fn runtime_test() {
        let mut x = compress(SHA224, [[0x8000_0000, 0], [0, 0]]);
        x[1] |= 0xFFFF_FFFF << 96;
        assert_eq!(
            x,
            [
                0x288234c4_476102bb_2a3a2bc9_d14a028c,
                0xFFFFFFFF_c5b3e42f_828ea62a_15a2b01f,
            ]
        );
    }
}
