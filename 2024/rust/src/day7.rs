use std::str::FromStr;

use num_bigint::BigInt;

use crate::parser;

#[derive(Debug)]
struct Equation {
    test_value: BigInt,
    numbers: Vec<BigInt>,
}

#[derive(Debug, Clone)]
struct ParseEquationError;

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let test_value = parts.next().unwrap().parse().unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Equation {
            test_value,
            numbers,
        })
    }
}

impl Equation {
    fn can_be_evaluated(&self) -> bool {
        self.find_evaluation(self.numbers.first().unwrap().clone(), 1)
    }

    fn find_evaluation(&self, total: BigInt, idx: usize) -> bool {
        if idx == self.numbers.len() {
            total == self.test_value
        } else {
            let next_number = &self.numbers[idx];
            self.find_evaluation(total.clone() + next_number, idx + 1)
                || self.find_evaluation(total.clone() * next_number, idx + 1)
        }
    }
}

pub fn part1() -> u32 {
    let equations: Vec<Equation> = parser::read("data/day7.txt").unwrap();
    let result = total_calibrations(&equations);
    println!("Result: {}", result);
    0
}

pub fn part2() -> u32 {
    0
}

fn total_calibrations(equations: &Vec<Equation>) -> BigInt {
    equations
        .iter()
        .filter(|e| e.can_be_evaluated())
        .map(|e| &e.test_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let equations = vec![
            Equation {
                test_value: 190.into(),
                numbers: vec![10.into(), 19.into()],
            },
            Equation {
                test_value: 3267.into(),
                numbers: vec![81.into(), 40.into(), 27.into()],
            },
            Equation {
                test_value: 81.into(),
                numbers: vec![17.into(), 15.into()],
            },
            Equation {
                test_value: 156.into(),
                numbers: vec![15.into(), 6.into()],
            },
            Equation {
                test_value: 7290.into(),
                numbers: vec![6.into(), 8.into(), 6.into(), 15.into()],
            },
            Equation {
                test_value: 161011.into(),
                numbers: vec![16.into(), 10.into(), 13.into()],
            },
            Equation {
                test_value: 192.into(),
                numbers: vec![17.into(), 8.into(), 14.into()],
            },
            Equation {
                test_value: 21037.into(),
                numbers: vec![9.into(), 7.into(), 18.into(), 13.into()],
            },
            Equation {
                test_value: 292.into(),
                numbers: vec![11.into(), 6.into(), 16.into(), 20.into()],
            },
        ];

        let result = total_calibrations(&equations);

        assert_eq!(result, 3749.into())
    }

    #[test]
    fn sample_input_part_2() {}
}
