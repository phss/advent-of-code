use std::str::FromStr;

use crate::parser;

struct Move {
    quantity: u32,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct ParseMoveError;

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, ParseMoveError> {
        let mut words = s.split_ascii_whitespace();

        words.next();
        let quantity: u32 = words.next().unwrap().parse().unwrap();

        words.next();
        let from: usize = words.next().unwrap().parse().unwrap();

        words.next();
        let to: usize = words.next().unwrap().parse().unwrap();

        Ok(Move { quantity, from, to})
    }
}


pub fn part1() -> u32 {
    let mut inital_crates = vec![
        vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'],
        vec!['J', 'R', 'V', 'L'],
        vec!['S', 'Q', 'F'],
        vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
        vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
        vec!['S', 'W', 'T', 'C', 'H', 'F'],
        vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'],
        vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
        vec!['J', 'B', 'W', 'V', 'P'],
    ];
    let moves: Vec<Move> = parser::read("data/day5.txt").unwrap();
    let crate_word = top_stack(&mut inital_crates, &moves);
    println!("{}", crate_word);
    0
}

pub fn part2() -> u32 {
    let mut inital_crates = vec![
        vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'],
        vec!['J', 'R', 'V', 'L'],
        vec!['S', 'Q', 'F'],
        vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
        vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
        vec!['S', 'W', 'T', 'C', 'H', 'F'],
        vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'],
        vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
        vec!['J', 'B', 'W', 'V', 'P'],
    ];
    let moves: Vec<Move> = parser::read("data/day5.txt").unwrap();
    let crate_word = top_stack_9001(&mut inital_crates, &moves);
    println!("{}", crate_word);
    0
}

fn top_stack(crates: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for create_move in moves {
        for _ in 0..create_move.quantity {
            let element = crates[create_move.from - 1].pop().unwrap();
            crates[create_move.to - 1].push(element);
        }
    }
    crates.into_iter().map(|c| c.pop().unwrap()).collect()
}

fn top_stack_9001(crates: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    for create_move in moves {
        let split_index = crates[create_move.from - 1].len() - create_move.quantity as usize;
        let mut elements = crates[create_move.from - 1].split_off(split_index);
        crates[create_move.to - 1].append(&mut elements);
    }
    crates.into_iter().map(|c| c.pop().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_top_stack() {
        let mut inital_crates = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let moves = vec![
            Move { quantity: 1, from: 2, to: 1 },
            Move { quantity: 3, from: 1, to: 3 },
            Move { quantity: 2, from: 2, to: 1 },
            Move { quantity: 1, from: 1, to: 2 },
        ];
        assert_eq!(top_stack(&mut inital_crates, &moves), "CMZ");
    }

    #[test]
    fn sample_input_top_stack_9001() {
        let mut inital_crates = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let moves = vec![
            Move { quantity: 1, from: 2, to: 1 },
            Move { quantity: 3, from: 1, to: 3 },
            Move { quantity: 2, from: 2, to: 1 },
            Move { quantity: 1, from: 1, to: 2 },
        ];
        assert_eq!(top_stack_9001(&mut inital_crates, &moves), "MCD");
    }
}
