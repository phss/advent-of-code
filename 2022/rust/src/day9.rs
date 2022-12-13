use std::{collections::HashSet, str::FromStr};

use crate::parser;

#[derive(Debug)]
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
    visited_positions(&moves, 1)
}

pub fn part2() -> u32 {
    let moves: Vec<Move> = parser::read("data/day9.txt").unwrap();
    visited_positions(&moves, 9)
}

fn visited_positions(moves: &Vec<Move>, rope_length: usize) -> u32 {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0)];
    for _ in 0..rope_length {
        rope.push((0, 0));
    }

    let mut visited = HashSet::new();
    visited.insert(rope.last().unwrap().clone());

    for rope_move in moves {
        println!("=== Move {:?} ===", rope_move);
        let ((x, y), steps) = match rope_move {
            Move::Right(distance) => ((1, 0), *distance),
            Move::Left(distance) => ((-1, 0), *distance),
            Move::Up(distance) => ((0, 1), *distance),
            Move::Down(distance) => ((0, -1), *distance),
        };

        for _ in 0..steps {
            rope[0].0 += x;
            rope[0].1 += y;

            for i in 1..rope.len() {
                if (rope[i-1].0 - rope[i].0) > 1 {
                    rope[i].0 += 1;
                    rope[i].1 -= (rope[i].1 - rope[i-1].1).clamp(-1, 1);
                } else if (rope[i-1].0 - rope[i].0) < -1 {
                    rope[i].0 -= 1;
                    rope[i].1 -= (rope[i].1 - rope[i-1].1).clamp(-1, 1);
                } else if (rope[i-1].1 - rope[i].1) > 1 {
                    rope[i].0 -= (rope[i].0 - rope[i-1].0).clamp(-1, 1);
                    rope[i].1 += 1;
                } else if (rope[i-1].1 - rope[i].1) < -1 {
                    rope[i].0 -= (rope[i].0 - rope[i-1].0).clamp(-1, 1);
                    rope[i].1 -= 1;
                }
            }

            visited.insert(rope.last().unwrap().clone());
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
        assert_eq!(visited_positions(&moves, 1), 13);
    }

    #[test]
    fn sample_input_part_2() {
        let moves = vec![
            Move::Right(5),
            Move::Up(8),
            Move::Left(8),
            Move::Down(3),
            Move::Right(17),
            Move::Down(10),
            Move::Left(25),
            Move::Right(20),
        ];
        assert_eq!(visited_positions(&moves, 9), 25);
    }
}
