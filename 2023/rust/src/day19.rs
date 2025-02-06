use itertools::Itertools;
use std::collections::HashMap;

use crate::parser;
use regex::Regex;

type Rule = (char, char, usize, String);

#[derive(Debug)]
struct Workflow {
    #[allow(dead_code)]
    label: String,
    rules: Vec<Rule>,
    fallback: String,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn ratings_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day19.txt").unwrap();
    let (workflows, parts) = parse(&lines);
    sum_of_accepted_parts(&workflows, &parts)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day19.txt").unwrap();
    let (workflows, _) = parse(&lines);
    all_accepted_combinations(&workflows, &"in".to_string(), &Vec::new())
}

fn sum_of_accepted_parts(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> usize {
    parts
        .iter()
        .filter(|part| is_accepted(workflows, part))
        .map(|part| part.ratings_sum())
        .sum()
}

fn is_accepted(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut current_node = &"in".to_string();
    let stop_nodes = vec!["A".to_string(), "R".to_string()];

    while !stop_nodes.contains(current_node) {
        let workflow = workflows.get(current_node).unwrap();
        let next_node = workflow
            .rules
            .iter()
            .find(|(attr, op, val, _)| {
                let actual = match attr {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => panic!("unreacheable attr"),
                };
                match op {
                    '<' => actual < *val,
                    '>' => actual > *val,
                    _ => panic!("unreacheable op"),
                }
            })
            .map(|(_, _, _, node)| node);
        current_node = next_node.unwrap_or(&workflow.fallback)
    }

    current_node == "A"
}

fn all_accepted_combinations(
    workflows: &HashMap<String, Workflow>,
    node: &String,
    previous_rules: &Vec<Rule>,
) -> usize {
    if node == &"R".to_string() {
        return 0;
    }
    if node == &"A".to_string() {
        return combinations(previous_rules);
    }

    let workflow = workflows.get(node).unwrap();

    let mut rule_combinations = 0;
    let mut negated_rules = Vec::new();
    for rule in &workflow.rules {
        let mut rules = previous_rules.clone();
        rules.extend(negated_rules.clone());
        rules.push(rule.clone());
        rule_combinations += all_accepted_combinations(workflows, &rule.3, &rules);
        negated_rules.push(negate(&rule));
    }

    let mut rules = previous_rules.clone();
    rules.extend(negated_rules);
    let fallback_combinations = all_accepted_combinations(workflows, &workflow.fallback, &rules);

    rule_combinations + fallback_combinations
}

fn negate((attr, op, val, _): &Rule) -> Rule {
    let (negated_op, negated_val) = if *op == '<' {
        ('>', val - 1)
    } else {
        ('<', val + 1)
    };
    (*attr, negated_op, negated_val, "ignored".to_string())
}

fn combinations(rules: &Vec<Rule>) -> usize {
    let rules_by_attr = rules.iter().into_group_map_by(|(attr, _, _, _)| attr);
    let grouped_rules: HashMap<_, _> = rules_by_attr
        .into_iter()
        .map(|(attr, rules)| {
            let start = rules
                .iter()
                .filter(|(_, op, _, _)| *op == '>')
                .map(|(_, _, val, _)| *val)
                .max()
                .unwrap_or(0);
            let end = rules
                .iter()
                .filter(|(_, op, _, _)| *op == '<')
                .map(|(_, _, val, _)| *val - 1)
                .min()
                .unwrap_or(4000);
            (attr, end - start)
        })
        .collect();

    let x = grouped_rules.get(&'x').unwrap_or(&4000);
    let m = grouped_rules.get(&'m').unwrap_or(&4000);
    let a = grouped_rules.get(&'a').unwrap_or(&4000);
    let s = grouped_rules.get(&'s').unwrap_or(&4000);

    x * m * a * s
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
            let field = rule_caps.get(1).unwrap().as_str().chars().next().unwrap();
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

        let result = sum_of_accepted_parts(&workflows, &parts);

        assert_eq!(result, 19114);
    }

    #[test]
    fn sample_input_part_2() {
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
        let (workflows, _) = parse(&lines);

        let result = all_accepted_combinations(&workflows, &"in".to_string(), &Vec::new());

        assert_eq!(result, 167409079868000);
    }
}
