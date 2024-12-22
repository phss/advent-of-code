use core::panic;

use regex::Regex;

use crate::parser;

#[derive(Debug, Clone)]
enum Instruction {
    Mult(u32, u32),
    Do,
    DoNot,
}

pub fn part1() -> u32 {
    let memory: Vec<String> = parser::read("data/day3.txt").unwrap();
    mult_corrupted(&memory)
}

pub fn part2() -> u32 {
    let memory: Vec<String> = parser::read("data/day3.txt").unwrap();
    mult_corrupted_with_flags(&memory)
}

fn mult_corrupted(memory: &Vec<String>) -> u32 {
    let mut result = 0;

    for instruction in parse_instructions(memory) {
        if let Instruction::Mult(x, y) = instruction {
            result += x * y;
        }
    }

    result
}

fn mult_corrupted_with_flags(memory: &Vec<String>) -> u32 {
    let mut enabled = true;
    let mut result = 0;

    for instruction in parse_instructions(memory) {
        match instruction {
            Instruction::Mult(x, y) => {
                if enabled {
                    result += x * y;
                }
            }
            Instruction::Do => enabled = true,
            Instruction::DoNot => enabled = false,
        }
    }

    result
}

fn parse_instructions(memory: &Vec<String>) -> Vec<Instruction> {
    let instructions_regexp = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut instructions = Vec::new();

    for line in memory {
        for capture in instructions_regexp.captures_iter(line) {
            let operation = capture.get(0).unwrap().as_str().split('(').next().unwrap();
            let instruction = match &operation {
                &"mul" => {
                    let x: u32 = capture[1].parse().unwrap();
                    let y: u32 = capture[2].parse().unwrap();
                    Instruction::Mult(x, y)
                }
                &"do" => Instruction::Do,
                &"don't" => Instruction::DoNot,
                invalid => panic!("Invalid instruction {}", invalid),
            };
            instructions.push(instruction);
        }
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let memory =
            vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];
        let memory: Vec<String> = memory.into_iter().map(|s| s.to_string()).collect();

        let result = mult_corrupted(&memory);

        assert_eq!(result, 161);
    }

    #[test]
    fn sample_input_part_2() {
        let memory =
            vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];
        let memory: Vec<String> = memory.into_iter().map(|s| s.to_string()).collect();

        let result = mult_corrupted_with_flags(&memory);

        assert_eq!(result, 48);
    }
}
