mod map;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::parser;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    position: (usize, usize),
    direction: (isize, isize),
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day16.txt").unwrap();
    let map = map::parse(&lines);
    lowest_score(&map)
}

pub fn part2() -> u32 {
    0
}

fn lowest_score(map: &Vec<Vec<char>>) -> u32 {
    let start_position = map::position(map, 'S');
    let end_position = map::position(map, 'E');

    let mut visited = HashSet::new();
    let mut search_heap = BinaryHeap::new();
    search_heap.push(Path {
        cost: 0,
        position: start_position,
        direction: (1, 0),
    });

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(Path {
        cost: current_cost,
        position,
        direction: current_direction,
    }) = search_heap.pop()
    {
        if position == end_position {
            return current_cost as u32;
        }

        let (x, y) = position;
        for direction @ (dir_x, dir_y) in directions {
            let next_x = x.wrapping_add_signed(dir_x);
            let next_y = y.wrapping_add_signed(dir_y);
            let next_position = (next_x, next_y);

            if map[next_y][next_x] == '#' || visited.contains(&next_position) {
                continue;
            }

            let mut move_cost = 1;
            if current_direction != direction {
                move_cost += 1000;
            }
            search_heap.push(Path {
                cost: current_cost + move_cost,
                position: next_position,
                direction,
            });

            visited.insert(next_position);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let map = map::parse(&lines);

        let result = lowest_score(&map);

        assert_eq!(result, 7036);
    }

    #[test]
    fn sample_input_part_1_another_example() {
        let lines = vec![
            "#################",
            "#...#...#...#..E#",
            "#.#.#.#.#.#.#.#.#",
            "#.#.#.#...#...#.#",
            "#.#.#.#.###.#.#.#",
            "#...#.#.#.....#.#",
            "#.#.#.#.#.#####.#",
            "#.#...#.#.#.....#",
            "#.#.#####.#.###.#",
            "#.#.#.......#...#",
            "#.#.###.#####.###",
            "#.#.#...#.....#.#",
            "#.#.#.#####.###.#",
            "#.#.#.........#.#",
            "#.#.#.#########.#",
            "#S#.............#",
            "#################",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let map = map::parse(&lines);

        let result = lowest_score(&map);

        assert_eq!(result, 11048);
    }

    #[test]
    fn sample_input_part_2() {}
}
