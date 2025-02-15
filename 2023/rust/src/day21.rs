use std::{collections::HashSet, usize};

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day21.txt").unwrap();
    let map = parse(&lines);
    plots_after_steps(&map, 64)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day21.txt").unwrap();
    let map = parse(&lines);
    calculate_solution(&map)
}

fn plots_after_steps(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let start = position(map, 'S');
    let mut visited = HashSet::new();
    visited.insert(start);

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for _ in 0..steps {
        let mut new_visited = HashSet::new();

        for (x, y) in visited {
            for (dir_x, dir_y) in directions {
                let next_x = x + dir_x;
                let next_y = y + dir_y;
                let next_position = (next_x, next_y);

                if new_visited.contains(&next_position)
                    || next_x < 0
                    || next_x >= width
                    || next_y < 0
                    || next_y >= height
                    || map[next_y as usize][next_x as usize] == '#'
                {
                    continue;
                }

                new_visited.insert(next_position);
            }
        }

        visited = new_visited;
    }

    visited.len()
}

fn calculate_solution(map: &Vec<Vec<char>>) -> usize {
    println!("{}", quadractic(1));
    println!("{}", quadractic(2));
    println!("{}", quadractic(3));
    println!("{}", quadractic(4));
    quadractic(25501365 / 131)
}

fn quadractic(x: usize) -> usize {
    14898 * x.pow(2) - 14799 * x + 3671
}

fn infinite_plots_after_steps(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let start = position(map, 'S');
    let mut visited = HashSet::new();
    visited.insert(start);

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for _ in 0..steps {
        let mut new_visited = HashSet::new();

        for (x, y) in visited {
            for (dir_x, dir_y) in directions {
                let next_x = x + dir_x;
                let next_y = y + dir_y;
                let next_position = (next_x, next_y);
                let adjusted_x = next_x.rem_euclid(width) as usize;
                let adjusted_y = next_y.rem_euclid(height) as usize;

                if new_visited.contains(&next_position) || map[adjusted_y][adjusted_x] == '#' {
                    continue;
                }

                new_visited.insert(next_position);
            }
        }

        visited = new_visited;
        // println!("{}", visited.len());
    }

    visited.len()
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

fn position(map: &Vec<Vec<char>>, char: char) -> (isize, isize) {
    let mut position = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == char) {
            position = (x as isize, y as isize);
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

    // #[test]
    // fn sample_input_part_2() {
    //     let lines = vec![
    //         "...........",
    //         ".....###.#.",
    //         ".###.##..#.",
    //         "..#.#...#..",
    //         "....#.#....",
    //         ".##..S####.",
    //         ".##..#...#.",
    //         ".......##..",
    //         ".##.#.####.",
    //         ".##..##.##.",
    //         "...........",
    //     ];
    //     let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
    //     let map = parse(&lines);

    //     let result = infinite_plots_after_steps(&map, 100);

    //     assert_eq!(result, 6536);
    // }
}
