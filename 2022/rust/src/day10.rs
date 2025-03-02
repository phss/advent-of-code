use std::{str::FromStr, vec};

use itertools::Itertools;

use crate::parser;

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, Clone)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, ParseInstructionError> {
        let mut splitted_line = s.split_whitespace();
        let instruction = splitted_line.next().unwrap();
        match instruction {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::Addx(
                splitted_line.next().map(|v| v.parse().unwrap()).unwrap(),
            )),
            _ => Err(ParseInstructionError),
        }
    }
}

pub fn part1() -> usize {
    let program: Vec<Instruction> = parser::read("data/day10.txt").unwrap();
    strength_signal_sum(&program) as usize
}

pub fn part2() -> usize {
    let program: Vec<Instruction> = parser::read("data/day10.txt").unwrap();
    print(&program);
    0
}

fn strength_signal_sum(program: &Vec<Instruction>) -> i32 {
    execution_register(program)
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(i, v)| i as i32 * v)
        .sum()
}

fn print(program: &Vec<Instruction>) {
    let pixels = execution_register(program)
        .iter()
        .skip(1)
        .enumerate()
        .map(|(cycle_index, value)| {
            let screen_index = (cycle_index % 40) as i32;
            if (value - 1..=value + 1).contains(&screen_index) {
                '#'
            } else {
                '.'
            }
        })
        .join("");

    println!("{}", &pixels[0..40]);
    println!("{}", &pixels[40..80]);
    println!("{}", &pixels[80..120]);
    println!("{}", &pixels[120..160]);
    println!("{}", &pixels[160..200]);
    println!("{}", &pixels[200..240]);
}

fn execution_register(program: &Vec<Instruction>) -> Vec<i32> {
    let mut register: Vec<i32> = vec![];
    let mut x = 1;
    register.push(x);

    for instruction in program {
        register.push(x);

        match instruction {
            Instruction::Addx(value) => {
                register.push(x);
                x += value
            }
            Instruction::Noop => {}
        }
    }

    register
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day10::Instruction::*;

    #[test]
    fn execution_cycles_output() {
        let program = vec![Noop, Addx(3), Addx(-5)];
        assert_eq!(execution_register(&program), vec![1, 1, 1, 1, 4, 4]);
    }

    #[test]
    fn sample_input_part_1() {
        let program: Vec<Instruction> = parser::read("data/day10test.txt").unwrap();
        assert_eq!(strength_signal_sum(&program), 13140);
    }

    #[test]
    fn sample_input_part_2() {
        let program: Vec<Instruction> = parser::read("data/day10test.txt").unwrap();
        print(&program);
    }
}
