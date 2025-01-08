mod map;
use std::collections::{HashMap, HashSet};

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day20.txt").unwrap();
    let map = map::parse(&lines);
    count_cheats(&map, 2, 100) as u32
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day20.txt").unwrap();
    let map = map::parse(&lines);
    count_cheats(&map, 20, 100) as u32
}

fn count_cheats(map: &Vec<Vec<char>>, cheat_steps: usize, time_saved: usize) -> u32 {
    let mut count = 0;
    let shortest_path = shortest_path(map, map::position(map, 'S'), map::position(map, 'E'));
    let shortest_path_steps = shortest_path.len() - 1;

    let mut steps_from_start = HashMap::new();
    let mut steps_to_end = HashMap::new();

    for (steps, position) in shortest_path.iter().enumerate() {
        steps_from_start.insert(position, steps);
        steps_to_end.insert(position, shortest_path_steps - steps);
    }

    for (a, start_to_a) in &steps_from_start {
        for (b, b_to_end) in &steps_to_end {
            let a_to_b = distance(a, b);
            if a_to_b <= cheat_steps {
                let cheat_distance = *start_to_a + *b_to_end + a_to_b;
                if shortest_path_steps as isize - cheat_distance as isize >= time_saved as isize {
                    count += 1;
                }
            }
        }
    }

    count
}

fn shortest_path(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    let mut position = start;
    while position != end {
        path.push(position);
        visited.insert(position);

        let (x, y) = position;
        for (dir_x, dir_y) in directions {
            let next_x = x.wrapping_add_signed(dir_x);
            let next_y = y.wrapping_add_signed(dir_y);
            let next_position = (next_x, next_y);

            if map[next_y][next_x] == '#' || visited.contains(&next_position) {
                continue;
            }

            position = next_position;
            break;
        }
    }
    path.push(end);

    path
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let (x1, y1) = *a;
    let (x2, y2) = *b;
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
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

        assert_eq!(count_cheats(&map, 2, 20), 5);
        assert_eq!(count_cheats(&map, 2, 12), 8);
    }

    #[test]
    fn sample_input_part_2() {}
}
