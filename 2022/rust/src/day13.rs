use std::{cmp::Ordering, fmt::Display};

use itertools::Itertools;

use crate::parser;

#[derive(PartialEq, Eq, Debug)]
enum Packet {
    List(Vec<Packet>),
    Value(i32),
}

impl Packet {
    fn parse(raw: &str) -> Self {
        let mut current_list: Vec<Packet> = vec![];
        let mut token = String::new();
        let mut stack = vec![];

        for c in raw.chars() {
            match c {
                '[' => {
                    stack.push(current_list);
                    current_list = vec![];
                }
                ']' | ',' => {
                    if !token.is_empty() {
                        let value: i32 = token.parse().unwrap();
                        current_list.push(Packet::Value(value));
                        token = String::new();
                    }
                    if c == ']' {
                        let mut previous_list = stack.pop().unwrap();
                        previous_list.push(Packet::List(current_list));
                        current_list = previous_list;
                    }
                }
                _ => {
                    token.push(c);
                }
            }
        }

        current_list.into_iter().next().unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(left_values), Packet::List(right_values)) => left_values
                .iter()
                .zip(right_values.iter())
                .find_map(|(left, right)| match left.cmp(right) {
                    Ordering::Equal => None,
                    result => Some(result),
                })
                .unwrap_or_else(|| {
                    Packet::Value(left_values.len() as i32)
                        .cmp(&Packet::Value(right_values.len() as i32))
                }),
            (Packet::List(_), Packet::Value(right_value)) => {
                self.cmp(&Packet::List(vec![Packet::Value(*right_value)]))
            }
            (Packet::Value(left_value), Packet::List(_)) => {
                Packet::List(vec![Packet::Value(*left_value)]).cmp(other)
            }
            (Packet::Value(left_value), Packet::Value(right_value)) => {
                if left_value < right_value {
                    Ordering::Less
                } else if left_value > right_value {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Packet::List(list_values) => {
                let values_as_string = list_values.iter().map(|value| value.to_string()).join(",");
                write!(f, "[{}]", values_as_string)
            }
            Packet::Value(value) => {
                write!(f, "{}", value)
            }
        }
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    let result: Vec<(usize, Ordering)> = lines
        .split(|line| line.is_empty())
        .map(|pair| {
            let mut pair_iter = pair.iter();
            let left = pair_iter.next().unwrap();
            let right = pair_iter.next().unwrap();
            Packet::parse(left).cmp(&Packet::parse(right))
        })
        .enumerate()
        .collect();

    result
        .iter()
        .filter(|(_, result)| result == &Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum::<usize>() as u32
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    decoder_key(&lines)
}

fn decoder_key(lines: &Vec<String>) -> u32 {
    let mut packets: Vec<Packet> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse(line))
        .collect();

    let first_divider = Packet::parse("[[2]]");
    let second_divider = Packet::parse("[[6]]");

    packets.push(first_divider);
    packets.push(second_divider);
    packets.sort();

    for packet in &packets {
        println!("{}", packet);
    }

    let first_divider_index = 1 + packets
        .iter()
        .position(|packet| packet == &Packet::parse("[[2]]"))
        .unwrap();
    let second_divider_index = 1 + packets
        .iter()
        .position(|packet| packet == &Packet::parse("[[6]]"))
        .unwrap();

    (first_divider_index * second_divider_index) as u32
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            Packet::parse("[1,2,[8,9],[[]],30,4,5]"),
            Packet::List(vec![
                Packet::Value(1),
                Packet::Value(2),
                Packet::List(vec![Packet::Value(8), Packet::Value(9),]),
                Packet::List(vec![Packet::List(vec![])]),
                Packet::Value(30),
                Packet::Value(4),
                Packet::Value(5)
            ])
        )
    }

    #[test]
    fn sample_input_part_1() {
        assert_eq!(
            Packet::parse("[1,1,3,1,1]").cmp(&Packet::parse("[1,1,5,1,1]")),
            Ordering::Less
        );
        assert_eq!(
            Packet::parse("[[1],[2,3,4]]").cmp(&Packet::parse("[[1],4]")),
            Ordering::Less
        );
        assert_eq!(
            Packet::parse("[9]").cmp(&Packet::parse("[[8,7,6]]")),
            Ordering::Greater
        );
        assert_eq!(
            Packet::parse("[[4,4],4,4]").cmp(&Packet::parse("[[4,4],4,4,4]")),
            Ordering::Less
        );
        assert_eq!(
            Packet::parse("[7,7,7,7]").cmp(&Packet::parse("[7,7,7]")),
            Ordering::Greater
        );
        assert_eq!(
            Packet::parse("[]").cmp(&Packet::parse("[3]")),
            Ordering::Less
        );
        assert_eq!(
            Packet::parse("[[[]]]").cmp(&Packet::parse("[[]]")),
            Ordering::Greater
        );
        assert_eq!(
            Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                .cmp(&Packet::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")),
            Ordering::Greater
        );
    }

    #[test]
    fn troubleshooting() {
        assert_eq!(
            Packet::parse("[1,1,3,1,1]").cmp(&Packet::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")),
            Ordering::Less
        );   
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            String::from("[1,1,3,1,1]"),
            String::from("[1,1,5,1,1]"),
            String::from(""),
            String::from("[[1],[2,3,4]]"),
            String::from("[[1],4]"),
            String::from(""),
            String::from("[9]"),
            String::from("[[8,7,6]]"),
            String::from(""),
            String::from("[[4,4],4,4]"),
            String::from("[[4,4],4,4,4]"),
            String::from(""),
            String::from("[7,7,7,7]"),
            String::from("[7,7,7]"),
            String::from(""),
            String::from("[]"),
            String::from("[3]"),
            String::from(""),
            String::from("[[[]]]"),
            String::from("[[]]"),
            String::from(""),
            String::from("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            String::from("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
        ];
        assert_eq!(decoder_key(&lines), 140)
    }
}
