mod map;
mod region;
use std::collections::HashSet;

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
            price += region.area() * region.perimeter(&map);
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
            price += region.area() * region.sides(&map);
            visited.extend(region.nodes);
        }
    }

    price
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
