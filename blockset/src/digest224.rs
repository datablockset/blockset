use crate::to_char;

pub type Digest224 = [u32; 7];

pub type Base32 = [u8; 45];

const fn get(digest: &Digest224, mut i: usize) -> (u8, usize, usize) {
    i *= 5;
    let d = i >> 5;
    let r = i & 0x1F;
    ((digest[d] >> r) as u8, d, r)
}

const fn one(digest: &Digest224, i: usize) -> u8 {
    let (mut x, d, r) = get(digest, i);
    let size = 32 - r;
    if size < 5 {
        x |= (digest[d + 1] << size) as u8
    }
    to_char(x)
}

pub const fn to_base32(digest: &Digest224) -> Base32 {
    let mut result = [0; 45];
    //
    result[0] = one(digest, 0);
    result[1] = one(digest, 1);
    result[2] = one(digest, 2);
    result[3] = one(digest, 3);
    result[4] = one(digest, 4);
    result[5] = one(digest, 5);
    result[6] = one(digest, 6);
    result[7] = one(digest, 7);
    result[8] = one(digest, 8);
    result[9] = one(digest, 9);
    //
    result[10] = one(digest, 10);
    result[11] = one(digest, 11);
    result[12] = one(digest, 12);
    result[13] = one(digest, 13);
    result[14] = one(digest, 14);
    result[15] = one(digest, 15);
    result[16] = one(digest, 16);
    result[17] = one(digest, 17);
    result[18] = one(digest, 18);
    result[19] = one(digest, 19);
    //
    result[20] = one(digest, 20);
    result[21] = one(digest, 21);
    result[22] = one(digest, 22);
    result[23] = one(digest, 23);
    result[24] = one(digest, 24);
    result[25] = one(digest, 25);
    result[26] = one(digest, 26);
    result[27] = one(digest, 27);
    result[28] = one(digest, 28);
    result[29] = one(digest, 29);
    //
    result[30] = one(digest, 30);
    result[31] = one(digest, 31);
    result[32] = one(digest, 32);
    result[33] = one(digest, 33);
    result[34] = one(digest, 34);
    result[35] = one(digest, 35);
    result[36] = one(digest, 36);
    result[37] = one(digest, 37);
    result[38] = one(digest, 38);
    result[39] = one(digest, 39);
    //
    result[40] = one(digest, 40);
    result[41] = one(digest, 41);
    result[42] = one(digest, 42);
    result[43] = one(digest, 43);
    result[44] = to_char(get(digest, 44).0);
    //
    result
}