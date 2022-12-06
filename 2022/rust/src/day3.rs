use std::collections::HashSet;

use crate::parser;

pub fn part1() -> u32 {
    let rucksacks: Vec<String> = parser::read("data/day3.txt").unwrap();
    sum_of_priorities(&rucksacks)
}

fn sum_of_priorities(rucksacks: &Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| common_priority(rucksack))
        .sum()
}

fn common_priority(rucksack: &String) -> u32 {
    let priorities: Vec<u32> = rucksack.chars().map(|c| priority(c)).collect();
    let (first_half, second_half) = priorities.split_at(priorities.len() / 2);

    let first_half: HashSet<&u32> = first_half.into_iter().collect();
    let second_half: HashSet<&u32> = second_half.into_iter().collect();

    **first_half.intersection(&second_half).next().unwrap()
}

fn priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        1 + (c as u32) - ('a' as u32)
    } else {
        27 + (c as u32) - ('A' as u32)
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
    fn priorities_conversion() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
