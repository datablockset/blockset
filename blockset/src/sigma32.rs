pub struct Big(u32, u32, u32);

impl Big {
    #[inline(always)]
    pub const fn get(&self, v: u32) -> u32 {
        v.rotate_right(self.0) ^ v.rotate_right(self.1) ^ v.rotate_right(self.2)
    }
}

pub struct Small(u32, u32, u8);

impl Small {
    #[inline(always)]
    pub const fn get(&self, v: u32) -> u32 {
        v.rotate_right(self.0) ^ v.rotate_right(self.1) ^ (v >> self.2)
    }
}

pub const BIG0: Big = Big(2, 13, 22);
pub const BIG1: Big = Big(6, 11, 25);
pub const SMALL0: Small = Small(7, 18, 3);
pub const SMALL1: Small = Small(17, 19, 10);
