use std::collections::HashMap;

use crate::parser;
use regex::Regex;

#[derive(Debug)]
struct Workflow {
    label: String,
    rules: Vec<(String, char, usize, String)>,
    fallback: String,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day19.txt").unwrap();
    let (workflows, parts) = parse(&lines);
    0
}

pub fn part2() -> usize {
    0
}

fn parse(lines: &Vec<String>) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut line_parts = lines.split(String::is_empty);

    let workflow_re = Regex::new(r"(\w+)\{([^}]+)\}").unwrap();
    let rules_re = Regex::new(r"(\w)([<>])(\d+):(\w+),?").unwrap();
    let mut workflows = HashMap::new();
    for line in line_parts.next().unwrap() {
        let caps = workflow_re.captures(line).unwrap();
        let label = caps.get(1).unwrap().as_str().to_string();
        let rules_str = caps.get(2).unwrap().as_str();

        let mut rules = Vec::new();
        for rule_caps in rules_re.captures_iter(rules_str) {
            let field = rule_caps.get(1).unwrap().as_str().to_string();
            let operator = rule_caps.get(2).unwrap().as_str().chars().next().unwrap();
            let value = rule_caps.get(3).unwrap().as_str().parse().unwrap();
            let target = rule_caps.get(4).unwrap().as_str().to_string();
            rules.push((field, operator, value, target));
        }

        let fallback = rules_str.split(",").last().unwrap().to_string();
        workflows.insert(
            label.clone(),
            Workflow {
                label,
                rules,
                fallback,
            },
        );
    }

    let part_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let mut parts = Vec::new();
    for line in line_parts.next().unwrap() {
        let caps = part_re.captures(line).unwrap();
        let x = caps.get(1).unwrap().as_str().parse().unwrap();
        let m = caps.get(2).unwrap().as_str().parse().unwrap();
        let a = caps.get(3).unwrap().as_str().parse().unwrap();
        let s = caps.get(4).unwrap().as_str().parse().unwrap();
        parts.push(Part { x, m, a, s });
    }
    (workflows, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "px{a<2006:qkq,m>2090:A,rfg}",
            "pv{a>1716:R,A}",
            "lnx{m>1548:A,A}",
            "rfg{s<537:gd,x>2440:R,A}",
            "qs{s>3448:A,lnx}",
            "qkq{x<1416:A,crn}",
            "crn{x>2662:A,R}",
            "in{s<1351:px,qqz}",
            "qqz{s>2770:qs,m<1801:hdj,R}",
            "gd{a>3333:R,R}",
            "hdj{m>838:A,pv}",
            "",
            "{x=787,m=2655,a=1222,s=2876}",
            "{x=1679,m=44,a=2067,s=496}",
            "{x=2036,m=264,a=79,s=2244}",
            "{x=2461,m=1339,a=466,s=291}",
            "{x=2127,m=1623,a=2188,s=1013}",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let (workflows, parts) = parse(&lines);

        // let result = sum_of_accepted_parts(&workflow, &parts);

        // assert_eq!(result, 19114);
    }

    #[test]
    fn sample_input_part_2() {}
}
