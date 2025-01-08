use std::collections::{HashMap, HashSet};

use crate::parser;

struct Solver {
    patterns: HashSet<String>,
    max_pattern_size: usize,
    cache: HashMap<usize, usize>,
}

impl Solver {
    fn new(patterns: &HashSet<String>) -> Self {
        let max_pattern_size = patterns.iter().map(String::len).max().unwrap_or(0);
        Solver {
            patterns: patterns.clone(),
            max_pattern_size,
            cache: HashMap::new(),
        }
    }

    fn is_valid_design(&mut self, start: usize, towel: &String) -> bool {
        self.design_permutations(start, towel) > 0
    }

    fn design_permutations(&mut self, start: usize, towel: &String) -> usize {
        if start >= towel.len() {
            return 1;
        } else if self.cache.contains_key(&start) {
            return *self.cache.get(&start).unwrap();
        }

        let mut result = 0;
        (1..=self.max_pattern_size).rev().for_each(|size| {
            let end = start + size;
            if end <= towel.len() {
                let sub_towel = &towel[start..end];
                if self.patterns.contains(sub_towel) {
                    result += self.design_permutations(end, towel);
                }
            }
        });
        self.cache.insert(start, result);

        result
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day19.txt").unwrap();
    let (patterns, towels) = parse(&lines);
    possible_designs(&patterns, &towels) as u32
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day19.txt").unwrap();
    let (patterns, towels) = parse(&lines);
    let result = all_possible_designs_ways_2000(&patterns, &towels);
    println!("Result {result}");
    0
}

fn possible_designs(patterns: &HashSet<String>, towels: &Vec<String>) -> usize {
    towels
        .iter()
        .filter(|towel| Solver::new(patterns).is_valid_design(0, towel))
        .count()
}

fn all_possible_designs_ways_2000(patterns: &HashSet<String>, towels: &Vec<String>) -> usize {
    towels
        .iter()
        .map(|towel| Solver::new(patterns).design_permutations(0, towel))
        .sum()
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
            "bggr",
            "gbbr",
            "rrbgbr",
            "ubwu",
            "bwurrg",
            "brgr",
            "bbrgwb",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (patterns, towels) = parse(&lines);

        let result = possible_designs(&patterns, &towels);

        assert_eq!(result, 6)
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "r, wr, b, g, bwu, rb, gb, br",
            "",
            "brwrr",
            "bggr",
            "gbbr",
            "rrbgbr",
            "ubwu",
            "bwurrg",
            "brgr",
            "bbrgwb",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (patterns, towels) = parse(&lines);

        let result = all_possible_designs_ways_2000(&patterns, &towels);

        assert_eq!(result, 16)
    }
}
