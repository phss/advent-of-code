use std::collections::{HashMap, HashSet};

use itertools::Itertools;

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

    pub fn sides(&self, map: &Map) -> u32 {
        let horizontal_directions = [(1, 0), (-1, 0)];
        let vertical_directions = [(0, 1), (0, -1)];
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut sides_by_direction = HashMap::new();
        for (x, y) in &self.nodes {
            for direction @ (dir_x, dir_y) in directions {
                let new_x = *x as isize + dir_x;
                let new_y = *y as isize + dir_y;

                let new_region = if new_x >= 0 && new_y >= 0 {
                    map.raw
                        .get(new_y as usize)
                        .unwrap_or(&String::new())
                        .chars()
                        .nth(new_x as usize)
                } else {
                    None
                };

                if new_region != Some(self.name) {
                    sides_by_direction
                        .entry(direction)
                        .or_insert(Vec::new())
                        .push((new_x, new_y));
                }
            }
        }

        let mut sides = 0;

        for direction in horizontal_directions {
            let blah = sides_by_direction
                .get(&direction)
                .unwrap()
                .iter()
                .into_group_map_by(|(x, _)| x)
                .into_values();
            for b in blah {
                sides += 1 + b
                    .iter()
                    .map(|(_, y)| y)
                    .sorted()
                    .tuple_windows()
                    .filter(|(a, b)| (*b - *a) > 1)
                    .count();
            }
        }

        for direction in vertical_directions {
            let blah = sides_by_direction
                .get(&direction)
                .unwrap()
                .iter()
                .into_group_map_by(|(_, y)| y)
                .into_values();
            for b in blah {
                sides += 1 + b
                    .iter()
                    .map(|(x, _)| x)
                    .sorted()
                    .tuple_windows()
                    .filter(|(a, b)| (*b - *a) > 1)
                    .count();
            }
        }

        sides as u32
    }
}
