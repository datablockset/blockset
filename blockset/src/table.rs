use crate::u224::U224;

pub trait Table {
    fn has_block(&self, key: &U224) -> bool;
    fn get_block(&self, key: &U224) -> Option<Vec<u8>>;
    fn set_block(&mut self, key: &U224, value: impl Iterator<Item = u8>) -> Option<()>;
}

pub struct Tables<M: Table, P: Table> {
    pub main: M,
    pub parts: P,
}
