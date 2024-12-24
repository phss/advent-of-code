use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

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

fn full_page_order(page_ordering_rules: &Vec<PageOrderingRule>) -> Vec<u32> {
    let all_pages: HashSet<u32> = page_ordering_rules
        .iter()
        .flat_map(|rule| vec![rule.before, rule.after])
        .collect();
    let before_count = page_ordering_rules.iter().counts_by(|rule| rule.after);

    let ordered_pages = all_pages
        .into_iter()
        .sorted_by_key(|page| before_count.get(page).unwrap_or(&0))
        .collect::<Vec<u32>>();

    ordered_pages
}

fn validate_page_ordering(
    page_ordering_rules: &Vec<PageOrderingRule>,
    pages_to_produce: &Vec<PagesToProduce>,
) -> u32 {
    let ordering = page_ordering_rules
        .into_iter()
        .fold(HashMap::new(), |mut acc, rule| {
            acc.entry(rule.before)
                .or_insert(HashSet::new())
                .insert(rule.after);
            acc
        });

    let mut result = 0;

    for pages in pages_to_produce {
        let mut valid = true;
        let mut previous_pages = HashSet::new();
        let empty_set = HashSet::new();

        for page in &pages.0 {
            let must_be_after_pages = ordering.get(page).unwrap_or(&empty_set);
            if !previous_pages.is_disjoint(must_be_after_pages) {
                valid = false;
                break;
            }
            previous_pages.insert(*page);
        }

        if valid {
            result += pages.middle_page();
        }
    }

    result
}

fn fixing_incorrect_page_ordering(
    page_ordering_rules: &Vec<PageOrderingRule>,
    pages_to_produce: &Vec<PagesToProduce>,
) -> u32 {
    let ordering = page_ordering_rules
        .into_iter()
        .fold(HashMap::new(), |mut acc, rule| {
            acc.entry(rule.before)
                .or_insert(HashSet::new())
                .insert(rule.after);
            acc
        });

    let mut result = 0;

    for pages in pages_to_produce {
        let mut previous_pages = HashSet::new();
        let empty_set = HashSet::new();

        for page in &pages.0 {
            let must_be_after_pages = ordering.get(page).unwrap_or(&empty_set);
            if !previous_pages.is_disjoint(must_be_after_pages) {
                let relevant_page_ordering_rules: Vec<PageOrderingRule> = page_ordering_rules
                    .iter()
                    .filter(|rule| pages.0.contains(&rule.before) && pages.0.contains(&rule.after))
                    .map(|rule| PageOrderingRule {
                        before: rule.before,
                        after: rule.after,
                    })
                    .collect();

                let blah = PagesToProduce(full_page_order(&relevant_page_ordering_rules));

                result += blah.middle_page();

                break;
            }
            previous_pages.insert(*page);
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

        // assert_eq!(result, 123)
    }
}
