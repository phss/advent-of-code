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
                let (area, perimeter, region_nodes) = price_for_region(map, region, position);

                price += area * perimeter;
                visited.extend(region_nodes);
            }
        }
    }

    price
}

fn price_for_region(
    map: &Vec<String>,
    region: char,
    start: (usize, usize),
) -> (u32, u32, HashSet<(usize, usize)>) {
    let mut region_nodes = HashSet::new();
    let mut perimeter = 0;
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
            } else {
                perimeter += 1;
            }
        }
    }

    (region_nodes.len() as u32, perimeter, region_nodes)
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
    fn sample_input_part_2() {}
}
