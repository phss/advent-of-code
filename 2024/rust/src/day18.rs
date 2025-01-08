use std::collections::{HashSet, VecDeque};

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day18.txt").unwrap();
    let bytes = parse(&lines);
    minimum_steps(&bytes, 1024, (71, 71))
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day18.txt").unwrap();
    let bytes = parse(&lines);
    let result = first_byte_to_block(&bytes, (71, 71));
    println!("Result {:?}", result);
    0
}

fn parse(lines: &Vec<String>) -> Vec<(usize, usize)> {
    lines
        .iter()
        .map(|line| {
            let elements: Vec<usize> = line.split(',').map(|e| e.parse().unwrap()).collect();
            (elements[0], elements[1])
        })
        .collect()
}

fn minimum_steps(
    bytes: &Vec<(usize, usize)>,
    bytes_fall: usize,
    dimensions @ (width, height): (usize, usize),
) -> u32 {
    let map = map_after_byte_fall(bytes, bytes_fall, dimensions);
    let start = (0, 0);
    let end = (width - 1, height - 1);

    shortest_path(&map, start, end, dimensions)
}

fn first_byte_to_block(
    bytes: &Vec<(usize, usize)>,
    dimensions @ (width, height): (usize, usize),
) -> (usize, usize) {
    let mut map = vec![vec!['.'; width]; height];
    let start = (0, 0);
    let end = (width - 1, height - 1);

    for byte @ (x, y) in bytes {
        map[*y][*x] = '#';

        if shortest_path(&map, start, end, dimensions) == 0 {
            return *byte;
        }
    }

    (0, 0)
}

fn shortest_path(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    (width, height): (usize, usize),
) -> u32 {
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
            let next_x = x.checked_add_signed(dir_x).unwrap_or(0).min(width - 1);
            let next_y = y.checked_add_signed(dir_y).unwrap_or(0).min(height - 1);
            let next_position = (next_x, next_y);

            if map[next_y][next_x] == '#'
                || position == next_position
                || visited.contains(&next_position)
            {
                continue;
            }

            search_queue.push_back((next_position, steps + 1));
            visited.insert(next_position);
        }
    }

    0
}

fn map_after_byte_fall(
    bytes: &Vec<(usize, usize)>,
    bytes_fall: usize,
    (width, height): (usize, usize),
) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; width]; height];

    (0..bytes_fall).for_each(|i| {
        let (x, y) = bytes[i];
        map[y][x] = '#';
    });

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1",
            "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6",
            "2,0",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let bytes = parse(&lines);

        let result = minimum_steps(&bytes, 12, (7, 7));

        assert_eq!(result, 22);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1",
            "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6",
            "2,0",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let bytes = parse(&lines);

        let result = first_byte_to_block(&bytes, (7, 7));

        assert_eq!(result, (6, 1));
    }
}
