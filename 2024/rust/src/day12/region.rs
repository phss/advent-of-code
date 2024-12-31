use std::collections::HashSet;

use super::map::Position;

pub struct Region {
    pub name: char,
    pub nodes: HashSet<Position>,
}

impl Region {
    pub fn area(&self) -> u32 {
        self.nodes.len() as u32
    }
}
