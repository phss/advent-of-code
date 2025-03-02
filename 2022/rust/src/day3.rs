use std::collections::HashSet;

use crate::parser;

pub fn part1() -> usize {
    let rucksacks: Vec<String> = parser::read("data/day3.txt").unwrap();
    sum_of_priorities(&rucksacks)
}

pub fn part2() -> usize {
    let rucksacks: Vec<String> = parser::read("data/day3.txt").unwrap();
    sum_of_badge_priorities(&rucksacks)
}

fn sum_of_priorities(rucksacks: &Vec<String>) -> usize {
    rucksacks
        .iter()
        .map(|rucksack| common_priority(rucksack))
        .sum()
}

fn common_priority(rucksack: &String) -> usize {
    let priorities: Vec<usize> = rucksack.chars().map(|c| priority(c)).collect();
    let (first_half, second_half) = priorities.split_at(priorities.len() / 2);

    let first_half: HashSet<&usize> = first_half.into_iter().collect();
    let second_half: HashSet<&usize> = second_half.into_iter().collect();

    **first_half.intersection(&second_half).next().unwrap()
}

fn sum_of_badge_priorities(rucksacks: &Vec<String>) -> usize {
    rucksacks
        .chunks(3)
        .map(|rucksacks| find_badge_priority(rucksacks))
        .sum()
}

fn find_badge_priority(rucksacks: &[String]) -> usize {
    let rucksacks: Vec<HashSet<usize>> = rucksacks
        .iter()
        .map(|rucksack| rucksack.chars().map(|c| priority(c)).collect())
        .collect();

    let ab: HashSet<&usize> = rucksacks[0].intersection(&rucksacks[1]).collect();
    let ab: HashSet<usize> = ab.iter().map(|v| **v).collect();
    *ab.intersection(&rucksacks[2]).next().unwrap()
}

fn priority(c: char) -> usize {
    if c >= 'a' && c <= 'z' {
        1 + (c as usize) - ('a' as usize)
    } else {
        27 + (c as usize) - ('A' as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_sum_of_priorities() {
        let rucksacks = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        assert_eq!(sum_of_priorities(&rucksacks), 157);
    }

    #[test]
    fn sample_input_sum_of_badge_priorities() {
        let rucksacks = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            String::from("ttgJtRGJQctTZtZT"),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        assert_eq!(sum_of_badge_priorities(&rucksacks), 70);
    }

    #[test]
    fn priorities_conversion() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
