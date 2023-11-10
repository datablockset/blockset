use std::io;

use crate::uint::{u224::U224, u256::U256};

pub trait TreeAddState {
    fn add_node(&mut self, node_id: &U256, main_height: usize) -> io::Result<u64>;
    fn end(&mut self, node_id: &U224, main_height: usize) -> io::Result<u64>;
}

impl TreeAddState for () {
    fn add_node(&mut self, _: &U256, _: usize) -> io::Result<u64> {
        Ok(0)
    }
    fn end(&mut self, _: &U224, _: usize) -> io::Result<u64> {
        Ok(0)
    }
}
