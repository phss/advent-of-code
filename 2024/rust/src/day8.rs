use std::collections::{HashMap, HashSet};

use crate::parser;
use itertools::Itertools;

pub fn part1() -> u32 {
    let map: Vec<String> = parser::read("data/day8.txt").unwrap();
    unique_antinodes_count(&map)
}

pub fn part2() -> u32 {
    0
}

fn unique_antinodes_count(map: &Vec<String>) -> u32 {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let antennas = find_antennas(map);
    let mut antinodes = HashSet::new();

    for positions in antennas.into_values() {
        for pair in positions.into_iter().combinations(2) {
            let ((a_x, a_y), (b_x, b_y)) = (pair[0], pair[1]);
            let x_diff = b_x as i32 - a_x as i32;
            let y_diff = b_y as i32 - a_y as i32;

            let antinode = (a_x as i32 - x_diff, a_y as i32 - y_diff);
            if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                antinodes.insert(antinode);
            }

            let antinode = (b_x as i32 + x_diff, b_y as i32 + y_diff);
            if antinode.0 >= 0 && antinode.0 < width && antinode.1 >= 0 && antinode.1 < height {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len() as u32
}

fn find_antennas(map: &Vec<String>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, content) in row.chars().enumerate() {
            if content != '.' {
                antennas
                    .entry(content)
                    .or_insert(Vec::new())
                    .push((row_idx, col_idx));
            }
        }
    }

    antennas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let map = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ];
        let map: Vec<String> = map.into_iter().map(|s| s.to_string()).collect();

        let result = unique_antinodes_count(&map);

        assert_eq!(result, 14)
    }

    #[test]
    fn sample_input_part_2() {}
}
