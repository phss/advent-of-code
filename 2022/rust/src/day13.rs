use itertools::Itertools;

use crate::parser;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
enum Comparison {
    RightOrder,
    WrongOrder,
    Same,
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    let result: Vec<(usize, Comparison)> = lines
        .split(|line| line.is_empty())
        .map(|pair| {
            let mut pair_iter = pair.iter();
            let left = pair_iter.next().unwrap();
            let right = pair_iter.next().unwrap();
            compare(&Packet::parse(left), &Packet::parse(right))
        })
        .enumerate()
        .collect();

    result
        .iter()
        .filter(|(_, result)| result == &Comparison::RightOrder)
        .map(|(i, _)| i + 1)
        .sum::<usize>() as u32
}

pub fn part2() -> u32 {
    0
}

fn compare(left: &Packet, right: &Packet) -> Comparison {
    // println!("{:?}", left);
    // println!("{:?}", right);
    // println!("");
    match (left, right) {
        (Packet::List(left_values), Packet::List(right_values)) => left_values
            .iter()
            .zip(right_values.iter())
            .find_map(|(left, right)| match compare(left, right) {
                Comparison::Same => None,
                result => Some(result),
            })
            .unwrap_or_else(|| {
                compare(
                    &Packet::Value(left_values.len() as i32),
                    &Packet::Value(right_values.len() as i32),
                )
            }),
        (Packet::List(_), Packet::Value(right_value)) => {
            compare(left, &Packet::List(vec![Packet::Value(*right_value)]))
        }
        (Packet::Value(left_value), Packet::List(_)) => {
            compare(&Packet::List(vec![Packet::Value(*left_value)]), right)
        }
        (Packet::Value(left_value), Packet::Value(right_value)) => {
            if left_value < right_value {
                Comparison::RightOrder
            } else if left_value > right_value {
                Comparison::WrongOrder
            } else {
                Comparison::Same
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
            compare(&Packet::parse("[1,1,3,1,1]"), &Packet::parse("[1,1,5,1,1]")),
            Comparison::RightOrder
        );
        assert_eq!(
            compare(&Packet::parse("[[1],[2,3,4]]"), &Packet::parse("[[1],4]")),
            Comparison::RightOrder
        );
        assert_eq!(
            compare(&Packet::parse("[9]"), &Packet::parse("[[8,7,6]]")),
            Comparison::WrongOrder
        );
        assert_eq!(
            compare(
                &Packet::parse("[[4,4],4,4]"),
                &Packet::parse("[[4,4],4,4,4]")
            ),
            Comparison::RightOrder
        );
        assert_eq!(
            compare(&Packet::parse("[7,7,7,7]"), &Packet::parse("[7,7,7]")),
            Comparison::WrongOrder
        );
        assert_eq!(
            compare(&Packet::parse("[]"), &Packet::parse("[3]")),
            Comparison::RightOrder
        );
        assert_eq!(
            compare(&Packet::parse("[[[]]]"), &Packet::parse("[[]]")),
            Comparison::WrongOrder
        );
        assert_eq!(
            compare(
                &Packet::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
                &Packet::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]")
            ),
            Comparison::WrongOrder
        );
    }

    #[test]
    fn sample_input_part_2() {}
}
