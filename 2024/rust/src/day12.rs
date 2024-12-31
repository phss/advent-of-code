use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::parser;

pub fn part1() -> u32 {
    let map: Vec<String> = parser::read("data/day12.txt").unwrap();
    total_price(&map)
}

pub fn part2() -> u32 {
    let map: Vec<String> = parser::read("data/day12.txt").unwrap();
    total_price_with_discount(&map)
}

fn total_price(map: &Vec<String>) -> u32 {
    let mut price = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        for (x, region) in row.chars().enumerate() {
            let position = (x, y);
            if !visited.contains(&position) {
                let region_nodes = get_region_nodes(map, region, position);

                let area = region_nodes.len() as u32;
                let perimeter = calculate_perimeter(map, region, &region_nodes);
                price += area * perimeter;

                visited.extend(region_nodes);
            }
        }
    }

    price
}

fn total_price_with_discount(map: &Vec<String>) -> u32 {
    let mut price = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (y, row) in map.iter().enumerate() {
        for (x, region) in row.chars().enumerate() {
            let position = (x, y);
            if !visited.contains(&position) {
                let region_nodes = get_region_nodes(map, region, position);

                let area = region_nodes.len() as u32;
                let sides = calculate_sides(map, region, &region_nodes);
                price += area * sides;

                visited.extend(region_nodes);
            }
        }
    }

    price
}

fn get_region_nodes(
    map: &Vec<String>,
    region: char,
    start: (usize, usize),
) -> HashSet<(usize, usize)> {
    let mut region_nodes = HashSet::new();
    let mut to_visit = vec![start];

    while let Some(position @ (x, y)) = to_visit.pop() {
        if region_nodes.contains(&position) {
            continue;
        }
        region_nodes.insert(position);

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dir_x, dir_y) in directions.iter() {
            let new_x = x.checked_add_signed(*dir_x);
            let new_y = y.checked_add_signed(*dir_y);
            let new_position = new_x.zip(new_y);

            let new_region = new_position.and_then(|(new_x, new_y)| {
                map.get(new_y).unwrap_or(&String::new()).chars().nth(new_x)
            });

            if new_region == Some(region) {
                to_visit.push(new_position.unwrap());
            }
        }
    }

    region_nodes
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
        let map = vec![
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
        let map: Vec<String> = map.into_iter().map(|s| s.to_string()).collect();

        let result = total_price(&map);

        assert_eq!(result, 1930)
    }

    #[test]
    fn sample_input_part_2() {
        let map = vec![
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
        let map: Vec<String> = map.into_iter().map(|s| s.to_string()).collect();

        let result = total_price_with_discount(&map);

        assert_eq!(result, 1206)
    }
}
