mod map;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::parser;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Path {
    cost: usize,
    position: (usize, usize),
    direction: (isize, isize),
    nodes: HashSet<(usize, usize)>,
}

impl Path {
    fn merge_nodes(&self, other_nodes: &HashSet<(usize, usize)>) -> Self {
        let mut merged_nodes = self.nodes.clone();
        merged_nodes.extend(other_nodes);
        Self {
            cost: self.cost,
            position: self.position,
            direction: self.direction,
            nodes: merged_nodes,
        }
    }
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
    let lines: Vec<String> = parser::read("data/day16.txt").unwrap();
    let map = map::parse(&lines);
    best_paths(&map)
}

fn lowest_score(map: &Vec<Vec<char>>) -> u32 {
    let (cost, _) = cost_and_paths(map, map::position(map, 'S'), map::position(map, 'E'));
    cost as u32
}

fn best_paths(map: &Vec<Vec<char>>) -> u32 {
    let (_, paths) = cost_and_paths(map, map::position(map, 'S'), map::position(map, 'E'));

    paths as u32
}

fn cost_and_paths(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> (usize, usize) {
    let mut visited: HashMap<((usize, usize), (isize, isize)), Path> = HashMap::new();
    let mut search_heap = BinaryHeap::new();

    search_heap.push(Path {
        cost: 0,
        position: start,
        direction: (1, 0),
        nodes: vec![start].into_iter().collect(),
    });

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(mut current_path) = search_heap.pop() {
        let visited_key = (current_path.position, current_path.direction);

        match visited.get(&visited_key) {
            Some(visited_path) if visited_path.cost < current_path.cost => continue,
            Some(visited_path) if visited_path.cost == current_path.cost => {
                current_path = current_path.merge_nodes(&visited_path.nodes);
                visited.insert(visited_key, current_path.clone())
            }
            _ => visited.insert(visited_key, current_path.clone()),
        };

        let (x, y) = current_path.position;
        for direction @ (dir_x, dir_y) in directions {
            let next_x = x.wrapping_add_signed(dir_x);
            let next_y = y.wrapping_add_signed(dir_y);
            let next_position = (next_x, next_y);

            if map[next_y][next_x] == '#' || current_path.nodes.contains(&next_position) {
                continue;
            }

            let mut move_cost = current_path.cost + 1;
            if direction != current_path.direction {
                move_cost += 1000;
            }

            let mut nodes = current_path.nodes.clone();
            nodes.insert(next_position);

            search_heap.push(Path {
                cost: move_cost,
                position: next_position,
                direction,
                nodes,
            });
        }
    }

    let end_node = visited
        .into_iter()
        .filter(|((node, _), _)| *node == end)
        .min_by_key(|(_, path)| path.cost)
        .map(|(_, path)| path)
        .unwrap();

    (end_node.cost, end_node.nodes.len())
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
    fn sample_input_part_2() {
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

        let result = best_paths(&map);

        assert_eq!(result, 45);
    }

    #[test]
    fn sample_input_part_2_another_example() {
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

        let result = best_paths(&map);

        assert_eq!(result, 64);
    }
}
