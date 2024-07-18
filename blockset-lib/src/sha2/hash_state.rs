use crate::uint::{
    u128x,
    u256x::{self, U256},
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
            len: self.len + 0x200,
        }
    }
    pub const fn end(mut self, mut data: U512, mut len: u16) -> U256 {
        assert!(len <= 0x200);
        if len == 0x200 {
            self = self.push(data);
            data = u512x::ZERO;
            len = 0;
        }
        {
            let q = len & 0x1F;
            let p = len & 0xFFE0;
            data = u512x::set_bit(data, (p | (0x1F - q)) as u32);
        }
        self.len += len as u64;
        let data11 = u128x::swap32(self.len as u128);
        if len < 0x1FF - 0x40 {
            data[1][1] |= data11;
            self.hash = compress(self.hash, data);
        } else {
            self.hash = compress(self.hash, data);
            self.hash = compress(self.hash, [u256x::ZERO, [0, data11]]);
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
        let f = |init, k, len| HashState::new(init).end(k, len);
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
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabc"
        // eaaf843057d4b3f741b4a19262164fb61adb0daf2c8196981696b414c7ad09fe
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x61626300]
                ],
                408
            ),
            u256x::swap32([
                0x1adb0da_f2c819698_1696b414_c7ad09fe,
                0xeaaf843_057d4b3f7_41b4a192_62164fb6
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcd"
        // 5622518d2df953c3e8506bd6d5c3a20f10d409afbb005ebec1b7ab15280dcfd6
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x61626364]
                ],
                416
            ),
            u256x::swap32([
                0x10d409a_fbb005ebe_c1b7ab15_280dcfd6,
                0x5622518_d2df953c3_e8506bd6_d5c3a20f
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcde"
        // e8c72b140c1b515b08e76dab90cd7b9483760a93767e30028a9fe94011d34c55
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x65000000_61626364]
                ],
                424
            ),
            u256x::swap32([
                0x83760a9_3767e3002_8a9fe940_11d34c55,
                0xe8c72b1_40c1b515b_08e76dab_90cd7b94
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdef"
        // 1979ef41c2bd877b3150fe6aba05372d9e1de8cd06a45918d4a75604b66026f3
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x65660000_61626364]
                ],
                432
            ),
            u256x::swap32([
                0x9e1de8c_d06a45918_d4a75604_b66026f3,
                0x1979ef4_1c2bd877b_3150fe6a_ba05372d
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefg"
        // c74c91051470cf0f398242e4832498da50b6fa22a9786a2924fe732c865616cc
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x65666700_61626364]
                ],
                440
            ),
            u256x::swap32([
                0x50b6fa2_2a9786a29_24fe732c_865616cc,
                0xc74c910_51470cf0f_398242e4_832498da
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefgh"
        // a32254a85e25153b03f9cd3ec2cfd74af080b3f5dd8bc2e73bbf9702923f5b5e
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [0x43444546_38394142_34353637_30313233, 0x65666768_61626364]
                ],
                448
            ),
            u256x::swap32([
                0xf080b3f_5dd8bc2e7_3bbf9702_923f5b5e,
                0xa32254a_85e25153b_03f9cd3e_c2cfd74a
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghi"
        // d4f8ba39f2bbf210e284c3df1af0f4a842d56f8d59a13f9ccbc762d97487ff0a
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x69000000_65666768_61626364
                    ]
                ],
                456
            ),
            u256x::swap32([
                0x42d56f8_d59a13f9c_cbc762d9_7487ff0a,
                0xd4f8ba3_9f2bbf210_e284c3df_1af0f4a8
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghij"
        // 87c074cbd39fe6f70f6cdee1652a0b5c87d443838c3110907c8fddb9ea45aa30
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x696A0000_65666768_61626364
                    ]
                ],
                464
            ),
            u256x::swap32([
                0x87d4438_38c311090_7c8fddb9_ea45aa30,
                0x87c074c_bd39fe6f7_0f6cdee1_652a0b5c
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghijk"
        // af2bd64ee47c437502fee60861488b70de1fb8a7f614c0c496974e2308703058
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x696A6B00_65666768_61626364
                    ]
                ],
                472
            ),
            u256x::swap32([
                0xde1fb8a_7f614c0c4_96974e23_08703058,
                0xaf2bd64_ee47c4375_02fee608_61488b70
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghijkl"
        // d470d6fbea20d21cecc15d3818442654885027e12f40568377524f512144c539
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x696A6B6C_65666768_61626364
                    ]
                ],
                480
            ),
            u256x::swap32([
                0x885027e_12f405683_77524f51_2144c539,
                0xd470d6f_bea20d21c_ecc15d38_18442654
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghijklm"
        // be0cdfaca8524e0de0725dcca4b0c78785bf82c7861903cb5e006128e4408265
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x6D000000_696A6B6C_65666768_61626364
                    ]
                ],
                488
            ),
            u256x::swap32([
                0x85bf82c_7861903cb_5e006128_e4408265,
                0xbe0cdfa_ca8524e0d_e0725dcc_a4b0c787
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghijklmn"
        // 245f1842136a9c656b54104352e734206d59546227dc233cdecb8ad70c2a944d
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x6D6E0000_696A6B6C_65666768_61626364
                    ]
                ],
                496
            ),
            u256x::swap32([
                0x6d59546_227dc233c_decb8ad7_0c2a944d,
                0x245f184_2136a9c65_6b541043_52e73420
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghijklmno"
        // 02198db64650f032738690585554acd9e9030a85b55d0ec46be30cb2ac05992c
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x6D6E6F00_696A6B6C_65666768_61626364
                    ]
                ],
                504
            ),
            u256x::swap32([
                0xe9030a8_5b55d0ec4_6be30cb2_ac05992c,
                0x02198db_64650f032_73869058_5554acd9
            ])
        );
        // "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEFabcdefghijklmnop"
        // ef8e2b127f816dee68cd063810d0976ade5e30b2ea59c47de2ac2c3a7b8f9471
        assert_eq!(
            f(
                SHA256,
                [
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x43444546_38394142_34353637_30313233
                    ],
                    [
                        0x43444546_38394142_34353637_30313233,
                        0x6D6E6F70_696A6B6C_65666768_61626364
                    ]
                ],
                512
            ),
            u256x::swap32([
                0xde5e30b_2ea59c47d_e2ac2c3a_7b8f9471,
                0xef8e2b1_27f816dee_68cd0638_10d0976a
            ])
        );
    }
}
