use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

use crate::parser;

struct PageOrderingRule {
    before: u32,
    after: u32,
}

#[derive(Debug, Clone)]
struct ParsePageOrderingRuleError;

impl FromStr for PageOrderingRule {
    type Err = ParsePageOrderingRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        Ok(PageOrderingRule {
            before: parts[0].parse().unwrap(),
            after: parts[1].parse().unwrap(),
        })
    }
}

struct PagesToProduce(Vec<u32>);

impl PagesToProduce {
    fn middle_page(&self) -> u32 {
        self.0[self.0.len() / 2]
    }
}

#[derive(Debug, Clone)]
struct ParsePagesToProduceError;

impl FromStr for PagesToProduce {
    type Err = ParsePagesToProduceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages: Vec<u32> = s.split(',').map(|page| page.parse().unwrap()).collect();
        Ok(PagesToProduce(pages))
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day5.txt").unwrap();
    let (page_ordering_rules, pages_to_produce) = parse(lines);
    validate_page_ordering(&page_ordering_rules, &pages_to_produce)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day5.txt").unwrap();
    let (page_ordering_rules, pages_to_produce) = parse(lines);
    fixing_incorrect_page_ordering(&page_ordering_rules, &pages_to_produce)
}

fn parse(lines: Vec<String>) -> (Vec<PageOrderingRule>, Vec<PagesToProduce>) {
    let mut parts = lines.split(|line| line.is_empty());
    let page_ordering_rules = parts
        .next()
        .unwrap()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let pages_to_produce = parts
        .next()
        .unwrap()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    (page_ordering_rules, pages_to_produce)
}

fn full_page_order(page_ordering_rules: &Vec<&PageOrderingRule>) -> Vec<u32> {
    let all_pages: HashSet<u32> = page_ordering_rules
        .iter()
        .flat_map(|rule| vec![rule.before, rule.after])
        .collect();
    let before_count = page_ordering_rules.iter().counts_by(|rule| rule.after);

    all_pages
        .into_iter()
        .sorted_by_key(|page| before_count.get(page).unwrap_or(&0))
        .collect::<Vec<u32>>()
}

fn validate_page_ordering(
    page_ordering_rules: &Vec<PageOrderingRule>,
    pages_to_produce: &Vec<PagesToProduce>,
) -> u32 {
    let mut result = 0;

    for pages in pages_to_produce {
        let relevant_page_ordering_rules = page_ordering_rules
            .iter()
            .filter(|rule| pages.0.contains(&rule.before) && pages.0.contains(&rule.after))
            .collect();

        let fully_ordered_pages = PagesToProduce(full_page_order(&relevant_page_ordering_rules));

        if pages.0 == fully_ordered_pages.0 {
            result += fully_ordered_pages.middle_page();
        }
    }

    result
}

fn fixing_incorrect_page_ordering(
    page_ordering_rules: &Vec<PageOrderingRule>,
    pages_to_produce: &Vec<PagesToProduce>,
) -> u32 {
    let mut result = 0;

    for pages in pages_to_produce {
        let relevant_page_ordering_rules = page_ordering_rules
            .iter()
            .filter(|rule| pages.0.contains(&rule.before) && pages.0.contains(&rule.after))
            .collect();

        let fully_ordered_pages = PagesToProduce(full_page_order(&relevant_page_ordering_rules));

        if pages.0 != fully_ordered_pages.0 {
            result += fully_ordered_pages.middle_page();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let page_ordering_rules: Vec<PageOrderingRule> = vec![
            "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29",
            "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61",
            "47|29", "75|13", "53|13",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
        let pages_to_produce: Vec<PagesToProduce> = vec![
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

        let result = validate_page_ordering(&page_ordering_rules, &pages_to_produce);

        assert_eq!(result, 143)
    }

    #[test]
    fn sample_input_part_2() {
        let page_ordering_rules: Vec<PageOrderingRule> = vec![
            "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29",
            "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61",
            "47|29", "75|13", "53|13",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
        let pages_to_produce: Vec<PagesToProduce> = vec![
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

        let result = fixing_incorrect_page_ordering(&page_ordering_rules, &pages_to_produce);

        assert_eq!(result, 123)
    }
}
