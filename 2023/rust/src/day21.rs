use std::collections::HashSet;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day21.txt").unwrap();
    let map = parse(&lines);
    plots_after_steps(&map, 64)
}

pub fn part2() -> usize {
    0
}

fn plots_after_steps(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let width = map[0].len();
    let height = map.len();

    let start = position(map, 'S');
    let mut visited = HashSet::new();
    visited.insert(start);

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for _ in 0..steps {
        let mut new_visited = HashSet::new();

        for (x, y) in visited {
            for (dir_x, dir_y) in directions {
                let next_x = x.checked_add_signed(dir_x).filter(|n| *n < width);
                let next_y = y.checked_add_signed(dir_y).filter(|n| *n < height);
                if next_x.is_none() || next_y.is_none() {
                    continue;
                }

                let next_x = next_x.unwrap();
                let next_y = next_y.unwrap();
                let next_position = (next_x, next_y);

                if new_visited.contains(&next_position) || map[next_y][next_x] == '#' {
                    continue;
                }

                new_visited.insert(next_position);
            }
        }

        visited = new_visited;
    }

    visited.len()
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

fn position(map: &Vec<Vec<char>>, char: char) -> (usize, usize) {
    let mut position = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == char) {
            position = (x, y);
            break;
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = plots_after_steps(&map, 6);

        assert_eq!(result, 16);
    }

    #[test]
    fn sample_input_part_2() {}
}
