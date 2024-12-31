mod map;
mod region;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use map::Map;

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day12.txt").unwrap();
    let map = Map { raw: lines };
    total_price(&map)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day12.txt").unwrap();
    let map = Map { raw: lines };
    total_price_with_discount(&map)
}

fn total_price(map: &Map) -> u32 {
    let mut price = 0;
    let mut visited = HashSet::new();

    for (_, position) in map.iter() {
        if !visited.contains(&position) {
            let region = map.get_region(position);

            let area = region.area();
            let perimeter = calculate_perimeter(&map.raw, region.name, &region.nodes);
            price += area * perimeter;

            visited.extend(region.nodes);
        }
    }

    price
}

fn total_price_with_discount(map: &Map) -> u32 {
    let mut price = 0;
    let mut visited = HashSet::new();

    for (_, position) in map.iter() {
        if !visited.contains(&position) {
            let region = map.get_region(position);

            let area = region.area();
            let sides = calculate_sides(&map.raw, region.name, &region.nodes);
            price += area * sides;

            visited.extend(region.nodes);
        }
    }

    price
}

fn calculate_perimeter(
    map: &Vec<String>,
    region: char,
    region_nodes: &HashSet<(usize, usize)>,
) -> u32 {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    region_nodes
        .iter()
        .flat_map(|(x, y)| {
            directions.iter().map(|(dir_x, dir_y)| {
                let new_x = x.checked_add_signed(*dir_x);
                let new_y = y.checked_add_signed(*dir_y);
                let new_position = new_x.zip(new_y);

                let new_region = new_position.and_then(|(new_x, new_y)| {
                    map.get(new_y).unwrap_or(&String::new()).chars().nth(new_x)
                });

                if new_region != Some(region) {
                    1
                } else {
                    0
                }
            })
        })
        .sum()
}

fn calculate_sides(map: &Vec<String>, region: char, region_nodes: &HashSet<(usize, usize)>) -> u32 {
    let horizontal_directions = [(1, 0), (-1, 0)];
    let vertical_directions = [(0, 1), (0, -1)];
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut sides_by_direction = HashMap::new();
    for (x, y) in region_nodes {
        for direction @ (dir_x, dir_y) in directions {
            let new_x = *x as isize + dir_x;
            let new_y = *y as isize + dir_y;

            let new_region = if new_x >= 0 && new_y >= 0 {
                map.get(new_y as usize)
                    .unwrap_or(&String::new())
                    .chars()
                    .nth(new_x as usize)
            } else {
                None
            };

            if new_region != Some(region) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let map = Map { raw: lines };

        let result = total_price(&map);

        assert_eq!(result, 1930)
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let map = Map { raw: lines };

        let result = total_price_with_discount(&map);

        assert_eq!(result, 1206)
    }
}
