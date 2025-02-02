use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    usize,
};

use crate::parser;

#[derive(Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    position: (usize, usize),
    direction: (isize, isize),
    straight: usize,
    visited: HashSet<(usize, usize)>,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day17.txt").unwrap();
    let map = parse(&lines);
    min_heat_loss(&map)
}

pub fn part2() -> usize {
    0
}

fn min_heat_loss(map: &Vec<Vec<usize>>) -> usize {
    let width = map[0].len();
    let height = map.len();

    let start_position = (0, 0);
    let end_position = (width - 1, height - 1);

    let mut visited = HashSet::new();
    let mut search_heap = BinaryHeap::new();
    search_heap.push(Path {
        cost: 0,
        position: start_position,
        direction: (0, 0),
        straight: 0,
        visited: HashSet::new(),
    });

    let mut lowest_cost = usize::MAX;
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(path) = search_heap.pop() {
        if path.position == end_position {
            lowest_cost = lowest_cost.min(path.cost);
            break;
        }

        let (x, y) = path.position;
        for direction @ (dir_x, dir_y) in directions {
            let next_x = x.checked_add_signed(dir_x).unwrap_or(0).min(width - 1);
            let next_y = y.checked_add_signed(dir_y).unwrap_or(0).min(height - 1);
            let next_position = (next_x, next_y);
            let straight = if direction == path.direction {
                path.straight + 1
            } else {
                1
            };

            if next_position == path.position
                || visited.contains(&(next_position, direction, straight))
                || path.visited.contains(&next_position)
                || straight > 3
            {
                continue;
            }

            let move_cost = map[next_y][next_x];
            let mut path_visited = path.visited.clone();
            path_visited.insert(next_position);
            search_heap.push(Path {
                cost: path.cost + move_cost,
                position: next_position,
                direction,
                straight,
                visited: path_visited,
            });

            visited.insert((next_position, direction, straight));
        }
    }

    lowest_cost
}

fn parse(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = min_heat_loss(&map);

        assert_eq!(result, 102);
    }

    #[test]
    fn sample_input_part_2() {}
}
