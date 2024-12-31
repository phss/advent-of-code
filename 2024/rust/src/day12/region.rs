use std::collections::HashSet;

use super::map::{Map, Position};

pub struct Region {
    pub name: char,
    pub nodes: HashSet<Position>,
}

impl Region {
    pub fn area(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn perimeter(&self, map: &Map) -> u32 {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        self.nodes
            .iter()
            .flat_map(|(x, y)| {
                directions.iter().map(|(dir_x, dir_y)| {
                    let new_x = x.checked_add_signed(*dir_x);
                    let new_y = y.checked_add_signed(*dir_y);
                    let new_position = new_x.zip(new_y);

                    let new_region = new_position.and_then(|(new_x, new_y)| {
                        map.raw
                            .get(new_y)
                            .unwrap_or(&String::new())
                            .chars()
                            .nth(new_x)
                    });

                    if new_region != Some(self.name) {
                        1
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}
