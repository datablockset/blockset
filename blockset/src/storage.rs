use std::io;

use crate::{u224::U224, u256::U256};

pub trait Storage {
    fn store(&mut self, key: &U256, level: usize) -> io::Result<()>;
    fn end(&mut self, key: &U224, level: usize) -> io::Result<()>;
}

pub struct Null();

impl Storage for Null {
    fn store(&mut self, _key: &U256, _level: usize) -> io::Result<()> { Ok(()) }
    fn end(&mut self, _key: &U224, _level: usize) -> io::Result<()> { Ok(()) }
}