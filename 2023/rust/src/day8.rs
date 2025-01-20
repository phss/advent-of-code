use std::collections::HashMap;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day8.txt").unwrap();
    let (moves, network) = parse(&lines);
    step_count(&moves, &network)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day8.txt").unwrap();
    let (moves, network) = parse(&lines);
    step_count_multiple(&moves, &network)
}

fn parse(lines: &Vec<String>) -> (Vec<char>, HashMap<String, (String, String)>) {
    let moves = lines[0].chars().collect();
    let mut network = HashMap::new();

    for line in lines.iter().skip(2) {
        if let Some((key, value)) = line.split_once(" = ") {
            let value = value.trim_matches(|c| c == '(' || c == ')');
            let mut parts = value.split(", ");
            let left = parts.next().unwrap().to_string();
            let right = parts.next().unwrap().to_string();
            network.insert(key.to_string(), (left, right));
        }
    }

    (moves, network)
}

fn step_count(moves: &Vec<char>, network: &HashMap<String, (String, String)>) -> usize {
    let mut node = "AAA".to_string();
    let mut i = 0;
    let mut steps = 0;

    while node != "ZZZ".to_string() {
        let (left, right) = network.get(&node).unwrap();

        node = if moves[i] == 'L' {
            left.clone()
        } else {
            right.clone()
        };

        i += 1;
        i = i % moves.len();
        steps += 1;
    }

    steps
}

fn step_count_multiple(moves: &Vec<char>, network: &HashMap<String, (String, String)>) -> usize {
    let mut nodes: Vec<String> = network
        .keys()
        .filter(|node| node.ends_with("A"))
        .cloned()
        .collect();
    let mut i = 0;
    let mut steps = 0;

    while nodes.iter().any(|node| !node.ends_with("Z")) {
        nodes.iter_mut().for_each(|node| {
            let (left, right) = network.get(node).unwrap();

            *node = if moves[i] == 'L' {
                left.clone()
            } else {
                right.clone()
            };
        });

        i += 1;
        i = i % moves.len();
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let (moves, network) = parse(&lines);

        let result = step_count(&moves, &network);

        assert_eq!(result, 2);
    }

    #[test]
    fn sample_input_part_1_longer_count() {
        let lines = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let (moves, network) = parse(&lines);

        let result = step_count(&moves, &network);

        assert_eq!(result, 6);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let (moves, network) = parse(&lines);

        let result = step_count_multiple(&moves, &network);

        assert_eq!(result, 6);
    }
}
