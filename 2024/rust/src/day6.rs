use std::collections::HashSet;

use crate::parser;

#[derive(Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1() -> u32 {
    let map: Vec<String> = parser::read("data/day6.txt").unwrap();
    distinct_walk_positions(&map)
}

pub fn part2() -> u32 {
    let map: Vec<String> = parser::read("data/day6.txt").unwrap();
    obstruction_research(&map)
}

fn distinct_walk_positions(map: &Vec<String>) -> u32 {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut locations = HashSet::new();
    let mut direction = Direction::Up;
    let mut current_location = starting_location(&map);

    loop {
        locations.insert(current_location);

        (direction, current_location) = move_one(&map, direction, current_location);

        let (next_row, next_col) = current_location;
        if next_row == -1 || next_row == height || next_col == -1 || next_col == width {
            break;
        }
    }

    locations.len() as u32
}

fn obstruction_research(map: &Vec<String>) -> u32 {
    let mut count = 0;

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, content) in row.chars().enumerate() {
            if content == '.' {
                let mut map_with_obstruction = map.clone();
                map_with_obstruction[row_idx].replace_range(col_idx..col_idx + 1, "#");

                if is_loop(&map_with_obstruction) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_loop(map: &Vec<String>) -> bool {
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    let mut locations = HashSet::new();
    let mut direction = Direction::Up;
    let mut location = starting_location(&map);

    loop {
        locations.insert((direction.clone(), location));

        (direction, location) = move_one(&map, direction, location);

        let (next_row, next_col) = location;
        if next_row == -1 || next_row == height || next_col == -1 || next_col == width {
            break;
        }

        if locations.contains(&(direction.clone(), location)) {
            return true;
        }
    }

    false
}

fn starting_location(map: &Vec<String>) -> (i32, i32) {
    let row = map.iter().position(|row| row.contains('^')).unwrap();
    let col = map[row].find('^').unwrap();

    (row as i32, col as i32)
}

fn move_one(
    map: &Vec<String>,
    current_direction: Direction,
    current_location: (i32, i32),
) -> (Direction, (i32, i32)) {
    let mut direction = current_direction;
    let mut location = current_location;
    let (row, col) = location;

    loop {
        location = match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        };

        let (next_row, next_col) = location;
        let next_content = map
            .get(next_row as usize)
            .unwrap_or(&String::new())
            .chars()
            .nth(next_col as usize);

        if next_content == Some('#') {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        } else {
            break;
        }
    }

    (direction, location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let map = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];
        let map: Vec<String> = map.into_iter().map(|s| s.to_string()).collect();

        let result = distinct_walk_positions(&map);

        assert_eq!(result, 41)
    }

    #[test]
    fn sample_input_part_2() {
        let map = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];
        let map: Vec<String> = map.into_iter().map(|s| s.to_string()).collect();

        let result = obstruction_research(&map);

        assert_eq!(result, 6)
    }
}
