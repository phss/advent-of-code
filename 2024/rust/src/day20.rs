mod map;
use std::collections::{HashSet, VecDeque};

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day20.txt").unwrap();
    let map = map::parse(&lines);
    count_cheats(&map, 100) as u32
}

pub fn part2() -> u32 {
    0
}

fn count_cheats(map: &Vec<Vec<char>>, time_saved: u32) -> u32 {
    let mut count = 0;
    let start = map::position(map, 'S');
    let end = map::position(map, 'E');
    let shortest_path_without_cheats = shortest_path(map, start, end);

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if map[y][x] == '#' {
                let mut cheated_map = map.clone();
                cheated_map[y][x] = 'x';
                let cheated_shortest_path = shortest_path(&cheated_map, start, end);
                if (shortest_path_without_cheats - cheated_shortest_path) >= time_saved {
                    count += 1;
                }
                println!("{:?} -> {}", (x, y), count);
            }
        }
    }

    count
}

fn shortest_path(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::new();
    let mut search_queue = VecDeque::new();
    search_queue.push_back((start, 0));
    while let Some((position, steps)) = search_queue.pop_front() {
        if position == end {
            return steps;
        }

        let (x, y) = position;
        for (dir_x, dir_y) in directions {
            let next_x = x.wrapping_add_signed(dir_x);
            let next_y = y.wrapping_add_signed(dir_y);
            let next_position = (next_x, next_y);

            if map[next_y][next_x] == '#' || visited.contains(&next_position) {
                continue;
            }

            search_queue.push_back((next_position, steps + 1));
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
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let map = map::parse(&lines);

        assert_eq!(count_cheats(&map, 20), 5);
    }

    #[test]
    fn sample_input_part_2() {}
}
