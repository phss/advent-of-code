use std::{collections::HashSet, str::FromStr};

use crate::parser;

enum Move {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Clone)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, ParseMoveError> {
        let mut splitted_line = s.split_whitespace();
        let direction = splitted_line.next().unwrap();
        let distance: i32 = splitted_line.next().unwrap().parse().unwrap();

        match direction {
            "R" => Ok(Move::Right(distance)),
            "L" => Ok(Move::Left(distance)),
            "U" => Ok(Move::Up(distance)),
            "D" => Ok(Move::Down(distance)),
            _ => Err(ParseMoveError),
        }
    }
}

pub fn part1() -> u32 {
    let moves: Vec<Move> = parser::read("data/day9.txt").unwrap();
    visited_positions(&moves)
}

pub fn part2() -> u32 {
    0
}

fn visited_positions(moves: &Vec<Move>) -> u32 {
    let mut visited = HashSet::new();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    visited.insert(tail);

    for rope_move in moves {
        let ((x, y), steps) = match rope_move {
            Move::Right(distance) => ((1, 0), *distance),
            Move::Left(distance) => ((-1, 0), *distance),
            Move::Up(distance) => ((0, 1), *distance),
            Move::Down(distance) => ((0, -1), *distance),
        };

        for _ in 0..steps {
            head.0 += x;
            head.1 += y;

            if (head.0 - tail.0) > 1 {
                tail.0 += 1;
                tail.1 = head.1;
            } else if (head.0 - tail.0) < -1 {
                tail.0 -= 1;
                tail.1 = head.1;
            } else if (head.1 - tail.1) > 1 {
                tail.0 = head.0;
                tail.1 += 1;
            } else if (head.1 - tail.1) < -1 {
                tail.0 = head.0;
                tail.1 -= 1;
            }

            visited.insert(tail);
        }
    }

    visited.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let moves = vec![
            Move::Right(4),
            Move::Up(4),
            Move::Left(3),
            Move::Down(1),
            Move::Right(4),
            Move::Down(1),
            Move::Left(5),
            Move::Right(2),
        ];
        assert_eq!(visited_positions(&moves), 13);
    }

    #[test]
    fn sample_input_part_2() {}
}
