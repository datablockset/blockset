use crate::uint::{
    u128x,
    u256x::U256,
    u512x::{self, U512},
};

use super::compress::compress;

pub struct HashState {
    hash: U256,
    len: u64,
}

impl HashState {
    pub const fn new(hash: U256) -> Self {
        Self { hash, len: 0 }
    }
    pub const fn push(self, data: U512) -> Self {
        Self {
            hash: compress(self.hash, data),
            len: self.len + 512,
        }
    }
    pub const fn end(mut self, mut data: U512, len: u16) -> U256 {
        assert!(len < 512);
        {
            let q = len & 0x1F;
            let p = len & 0xFFE0;
            data = u512x::set_bit(data, (p | (0x1F - q)) as u32);
        }
        self.len += len as u64;
        let data11 = u128x::swap32(self.len as u128);
        if len < 511 - 64 {
            data[1][1] |= data11;
            self.hash = compress(self.hash, data);
        } else {
            self.hash = compress(self.hash, data);
            self.hash = compress(self.hash, [[0, 0], [0, data11]]);
        }
        self.hash
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{
        sha2::{sha224::SHA224, sha256::SHA256},
        uint::{u256x, u512x},
    };

    use super::HashState;

    #[test]
    #[wasm_bindgen_test]
    fn test() {
        let f = |init, k, len| {
            let state = HashState::new(init);
            state.end(k, len)
        };
        // d14a028c_2a3a2bc9_476102bb_288234c4
        // 15a2b01f_828ea62a_c5b3e42f
        {
            let mut h = f(SHA224, u512x::ZERO, 0);
            h[1] |= 0xFFFF_FFFF << 96;
            assert_eq!(
                h,
                [
                    0x288234c4_476102bb_2a3a2bc9_d14a028c,
                    0xFFFFFFFF_c5b3e42f_828ea62a_15a2b01f,
                ]
            );
        }
        // e3b0c442_98fc1c14_9afbf4c8_996fb924
        // 27ae41e4_649b934c_a495991b_7852b855
        assert_eq!(
            f(SHA256, u512x::ZERO, 0),
            [
                0x996fb924_9afbf4c8_98fc1c14_e3b0c442,
                0x7852b855_a495991b_649b934c_27ae41e4,
            ],
        );
        // "0"
        // 5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9
        assert_eq!(
            f(SHA256, [[0x3000_0000, 0], [0, 0]], 8),
            u256x::swap32([
                0xc2dbc23_9dd4e91b4_6729d73a_27fb57e9,
                0x5feceb6_6ffc86f38_d952786c_6d696c79,
            ])
        );
        // "01"
        // 938db8c9f82c8cb58d3f3ef4fd250036a48d26a712753d2fde5abd03a85cabf4
        assert_eq!(
            f(SHA256, [[0x3031_0000, 0], [0, 0]], 16),
            u256x::swap32([
                0xa48d26a_712753d2f_de5abd03_a85cabf4,
                0x938db8c_9f82c8cb5_8d3f3ef4_fd250036
            ])
        );
        // "012"
        // bf6aaaab7c143ca12ae448c69fb72bb4cf1b29154b9086a927a0a91ae334cdf7
        assert_eq!(
            f(SHA256, [[0x3031_3200, 0], [0, 0]], 24),
            u256x::swap32([
                0xcf1b291_54b9086a9_27a0a91a_e334cdf7,
                0xbf6aaaa_b7c143ca1_2ae448c6_9fb72bb4
            ])
        );
        // "0123"
        // 1be2e452b46d7a0d9656bbb1f768e8248eba1b75baed65f5d99eafa948899a6a
        assert_eq!(
            f(SHA256, [[0x3031_3233, 0], [0, 0]], 32),
            u256x::swap32([
                0x8eba1b7_5baed65f5_d99eafa9_48899a6a,
                0x1be2e45_2b46d7a0d_9656bbb1_f768e824
            ])
        );
        // "01234"
        // c565fe03ca9b6242e01dfddefe9bba3d98b270e19cd02fd85ceaf75e2b25bf12
        assert_eq!(
            f(SHA256, [[0x3400_0000_3031_3233, 0], [0, 0]], 40),
            u256x::swap32([
                0x98b270e_19cd02fd8_5ceaf75e_2b25bf12,
                0xc565fe0_3ca9b6242_e01dfdde_fe9bba3d
            ])
        );
        // "01234567"
        // 924592b9b103f14f833faafb67f480691f01988aa457c0061769f58cd47311bc
        assert_eq!(
            f(SHA256, [[0x3435_3637_3031_3233, 0], [0, 0]], 64),
            u256x::swap32([
                0x1f01988_aa457c006_1769f58c_d47311bc,
                0x924592b_9b103f14f_833faafb_67f48069
            ])
        );
        // "0123456789ABCDEF"
        // 2125b2c332b1113aae9bfc5e9f7e3b4c91d828cb942c2df1eeb02502eccae9e9
        assert_eq!(
            f(
                SHA256,
                [[0x43444546_38394142_34353637_30313233, 0], [0, 0]],
                128
            ),
            u256x::swap32([
                0x91d828c_b942c2df1_eeb02502_eccae9e9,
                0x2125b2c_332b1113a_ae9bfc5e_9f7e3b4c
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF"
        // cd6c1f7d1dc6717d6371d2647910ca71ba3bf0b611083d322466b8843b4285b6
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0, 0]
                ],
                256
            ),
            u256x::swap32([
                0xba3bf0b_611083d32_2466b884_3b4285b6,
                0xcd6c1f7_d1dc6717d_6371d264_7910ca71
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF"
        // a2d094d2605d33b19a0c75f3aa4b5dc1eeacba0068799289f2a0960e755e5cd2
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0]
                ],
                384
            ),
            u256x::swap32([
                0xeeacba0_068799289_f2a0960e_755e5cd2,
                0xa2d094d_2605d33b1_9a0c75f3_aa4b5dc1
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFa"
        // 4ae493e89db8ecc2b52f49cd0c0bb6f3d68793733e84347005ba8fb59fc653bf
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x61000000]
                ],
                392
            ),
            u256x::swap32([
                0xd687937_33e843470_05ba8fb5_9fc653bf,
                0x4ae493e_89db8ecc2_b52f49cd_0c0bb6f3
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFab"
        // 3cba8dc04c46e175ade60333067a631cf5d5804610e8679800014ffbbb00b877
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x61620000]
                ],
                400
            ),
            u256x::swap32([
                0xf5d5804_610e86798_00014ffb_bb00b877,
                0x3cba8dc_04c46e175_ade60333_067a631c
            ])
        );
    }
}
