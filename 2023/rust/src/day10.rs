mod map;
use std::{
    collections::{HashSet, VecDeque},
    ops::Neg,
};

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day10.txt").unwrap();
    let map = map::parse(&lines);
    furthest_steps(&map)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day10.txt").unwrap();
    let map = map::parse(&lines);
    enclosed(&map)
}

fn furthest_steps(map: &Vec<Vec<char>>) -> usize {
    find_loop(map).len() / 2
}

fn find_loop(map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let width = map[0].len();
    let height = map.len();

    let start = map::position(map, 'S');
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut search_heap = VecDeque::new();
    search_heap.push_back(start);
    search_heap.push_back(start);

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some((x, y)) = search_heap.pop_front() {
        for direction @ (dir_x, dir_y) in directions {
            let next_x = x.checked_add_signed(dir_x).unwrap_or(0).min(width - 1);
            let next_y = y.checked_add_signed(dir_y).unwrap_or(0).min(height - 1);
            let next_position = (next_x, next_y);

            let from = map[y][x];
            let to = map[next_y][next_x];

            if visited.contains(&next_position)
                || !valid_step(from, (dir_x.neg(), dir_y.neg()))
                || !valid_step(to, direction)
            {
                continue;
            }

            visited.insert(next_position);
            search_heap.push_back(next_position);
        }
    }

    visited
}

fn valid_step(pipe: char, (x, y): (isize, isize)) -> bool {
    match (pipe, x, y) {
        ('S', _, _) => true,
        ('|', _, 1) => true,
        ('|', _, -1) => true,
        ('-', 1, _) => true,
        ('-', -1, _) => true,
        ('L', _, 1) => true,
        ('L', -1, _) => true,
        ('J', _, 1) => true,
        ('J', 1, _) => true,
        ('7', _, -1) => true,
        ('7', 1, _) => true,
        ('F', _, -1) => true,
        ('F', -1, _) => true,
        _ => false,
    }
}

fn enclosed(map: &Vec<Vec<char>>) -> usize {
    let width = map[0].len();
    let height = map.len();
    let loop_nodes = find_loop(map);
    let mut inside = 0;

    for y in 0..height {
        let mut left_pipes = 0;
        let mut previous_twisted_pipe = ' ';
        for x in 0..width {
            if loop_nodes.contains(&(x, y)) {
                match (previous_twisted_pipe, map[y][x]) {
                    (_, '-') => {}
                    (_, '|') => left_pipes += 1,
                    ('L', '7') => {
                        left_pipes += 1;
                        previous_twisted_pipe = ' ';
                    }
                    ('S', '7') => {
                        left_pipes += 1;
                        previous_twisted_pipe = ' ';
                    }
                    ('F', 'J') => {
                        left_pipes += 1;
                        previous_twisted_pipe = ' ';
                    }
                    (_, pipe) => previous_twisted_pipe = pipe,
                }
            } else if left_pipes % 2 != 0 {
                inside += 1;
            }
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec!["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = map::parse(&lines);

        let result = furthest_steps(&map);

        assert_eq!(result, 8);
    }

    #[test]
    fn sample_input_part_1_small_loop() {
        let lines = vec!["-L|F7", "7S-7|", "L|7||", "-L-J|", "L|-JF"];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = map::parse(&lines);

        let result = furthest_steps(&map);

        assert_eq!(result, 4);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = map::parse(&lines);

        let result = enclosed(&map);

        assert_eq!(result, 10);
    }
}
