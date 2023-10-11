use crate::u256::U256;

pub trait Storage {
    fn store(&mut self, key: U256, level: usize);
    fn end(&mut self);
}

pub struct Null();

impl Storage for Null {
    fn store(&mut self, _key: U256, _level: usize) {}
    fn end(&mut self) {}
}
