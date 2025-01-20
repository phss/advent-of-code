use std::collections::HashMap;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day8.txt").unwrap();
    let (moves, network) = parse(&lines);
    step_count(&"AAA".to_string(), &moves, &network)
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

fn step_count(
    initial: &String,
    moves: &Vec<char>,
    network: &HashMap<String, (String, String)>,
) -> usize {
    let mut node = initial.clone();
    let mut i = 0;
    let mut steps = 0;

    while !node.ends_with("Z") {
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
    let nodes: Vec<String> = network
        .keys()
        .filter(|node| node.ends_with("A"))
        .cloned()
        .collect();

    let steps_to_z: Vec<usize> = nodes
        .iter()
        .map(|node| step_count(node, moves, network))
        .collect();

    lcm(&steps_to_z)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

        let result = step_count(&"AAA".to_string(), &moves, &network);

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

        let result = step_count(&"AAA".to_string(), &moves, &network);

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
