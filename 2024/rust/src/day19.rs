use std::collections::HashSet;

use crate::parser;

struct Solver {
    patterns: HashSet<String>,
    max_pattern_size: usize,
}

impl Solver {
    fn is_valid_design(&self, start: usize, towel: &String) -> bool {
        if start >= towel.len() {
            return true;
        }

        (1..=self.max_pattern_size).rev().any(|size| {
            let end = start + size;
            if end <= towel.len() {
                let sub_towel = &towel[start..end];
                self.patterns.contains(sub_towel) && self.is_valid_design(end, towel)
            } else {
                false
            }
        })
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day19.txt").unwrap();
    let (patterns, towels) = parse(&lines);
    possible_designs(&patterns, &towels) as u32
}

pub fn part2() -> u32 {
    0
}

fn possible_designs(patterns: &HashSet<String>, towels: &Vec<String>) -> usize {
    let max_pattern_size = patterns.iter().map(String::len).max().unwrap();
    let solver = Solver {
        patterns: patterns.clone(),
        max_pattern_size,
    };
    towels
        .iter()
        .filter(|towel| solver.is_valid_design(0, towel))
        .count()
}

fn parse(lines: &Vec<String>) -> (HashSet<String>, Vec<String>) {
    let mut parts = lines.split(|line| line.is_empty());
    let patterns = parts
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let towels = parts
        .next()
        .unwrap()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    (patterns, towels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "r, wr, b, g, bwu, rb, gb, br",
            "",
            "brwrr",
            // "bggr",
            // "gbbr",
            // "rrbgbr",
            // "ubwu",
            // "bwurrg",
            // "brgr",
            // "bbrgwb",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (patterns, towels) = parse(&lines);

        let result = possible_designs(&patterns, &towels);

        assert_eq!(result, 6)
    }

    #[test]
    fn sample_input_part_2() {}
}
