use std::collections::HashSet;

use crate::parser;

pub fn part1() -> u32 {
    let map: Vec<String> = parser::read("data/day12.txt").unwrap();
    total_price(&map)
}

pub fn part2() -> u32 {
    0
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
    0
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

        // assert_eq!(result, 1206)
    }
}
