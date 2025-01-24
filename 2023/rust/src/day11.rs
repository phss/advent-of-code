mod map;
use std::collections::HashSet;

use crate::parser;
use itertools::Itertools;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day11.txt").unwrap();
    let map = map::parse(&lines);
    sum_of_lengths_after_expansion(&map)
}

pub fn part2() -> usize {
    0
}

fn sum_of_lengths_after_expansion(map: &Vec<Vec<char>>) -> usize {
    let mut lengths = 0;
    let galaxies = map::positions(map, '#');

    let cols_with_galaxy: HashSet<usize> = galaxies.iter().map(|(x, _)| x).cloned().collect();
    let rows_with_galaxy: HashSet<usize> = galaxies.iter().map(|(_, y)| y).cloned().collect();

    for pair in galaxies.iter().combinations(2) {
        let (x_a, y_a) = *pair[0];
        let (x_b, y_b) = *pair[1];

        let mut length = (x_a as isize - x_b as isize).abs() + (y_a as isize - y_b as isize).abs();

        (x_a.min(x_b)..x_a.max(x_b)).for_each(|x| {
            if !cols_with_galaxy.contains(&x) {
                length += 1;
            }
        });

        (y_a.min(y_b)..y_a.max(y_b)).for_each(|y| {
            if !rows_with_galaxy.contains(&y) {
                length += 1;
            }
        });

        lengths += length;
    }

    lengths as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = map::parse(&lines);

        let result = sum_of_lengths_after_expansion(&map);

        assert_eq!(result, 374);
    }

    #[test]
    fn sample_input_part_2() {}
}
