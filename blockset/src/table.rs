use crate::u224::U224;

pub trait Table {
    fn has(&self, key: &U224) -> bool;
    fn get(&self, key: &U224) -> Option<Vec<u8>>;
    fn set(&mut self, key: &U224, value: impl Iterator<Item = u8>);
}
