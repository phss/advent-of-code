use regex::Regex;

use crate::parser;

pub fn part1() -> u32 {
    let memory: Vec<String> = parser::read("data/day3.txt").unwrap();
    mult_corrupted(&memory)
}

pub fn part2() -> u32 {
    0
}

fn mult_corrupted(memory: &Vec<String>) -> u32 {
    let mul_regexp = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;

    for line in memory {
        for capture in mul_regexp.captures_iter(line) {
            let x: u32 = capture[1].parse().unwrap();
            let y: u32 = capture[2].parse().unwrap();
            result += x * y;
        }
    }

    result
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
    fn sample_input_part_2() {}
}
