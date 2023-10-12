use crate::u224::U224;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Main = 0,
    Parts = 1,
}

pub trait Table {
    fn has_block(&self, t: Type, key: &U224) -> bool;
    fn get_block(&self, t: Type, key: &U224) -> Option<Vec<u8>>;
    fn set_block(&mut self, t: Type, key: &U224, value: impl Iterator<Item = u8>) -> Option<()>;
}
