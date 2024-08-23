use std::io;

use crate::uint::{u224x::U224, u256x::U256};

pub trait TreeAdd {
    fn push(&mut self, node_id: &U256, main_height: usize) -> io::Result<u64>;
    fn end(&mut self, node_id: &U224, main_height: usize) -> io::Result<u64>;
}

impl TreeAdd for () {
    fn push(&mut self, _: &U256, _: usize) -> io::Result<u64> {
        Ok(0)
    }
    fn end(&mut self, _: &U224, _: usize) -> io::Result<u64> {
        Ok(0)
    }
}
