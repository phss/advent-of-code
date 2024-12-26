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

type Operation = fn(BigInt, BigInt) -> BigInt;

impl Equation {
    const ADD: fn(BigInt, BigInt) -> BigInt = |a: BigInt, b: BigInt| a + b;
    const MULT: fn(BigInt, BigInt) -> BigInt = |a: BigInt, b: BigInt| a * b;
    const CONCAT: fn(BigInt, BigInt) -> BigInt = |a: BigInt, b: BigInt| {
        let mut new_value = a.to_string();
        new_value.push_str(&b.to_string());
        new_value.parse().unwrap()
    };

    fn can_be_evaluated_with_add_and_mult(&self) -> bool {
        let operations = vec![Equation::ADD, Equation::MULT];
        self.any_combination_evaluate(self.numbers.first().unwrap().clone(), 1, &operations)
    }

    fn can_be_evaluated_with_all_ops(&self) -> bool {
        let operations = vec![Equation::ADD, Equation::MULT, Equation::CONCAT];
        self.any_combination_evaluate(self.numbers.first().unwrap().clone(), 1, &operations)
    }

    fn any_combination_evaluate(
        &self,
        total: BigInt,
        idx: usize,
        operations: &Vec<Operation>,
    ) -> bool {
        if idx == self.numbers.len() {
            total == self.test_value
        } else if total > self.test_value {
            false
        } else {
            let next_number = &self.numbers[idx];
            operations.iter().any(|op| {
                self.any_combination_evaluate(
                    op(total.clone(), next_number.clone()),
                    idx + 1,
                    operations,
                )
            })
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
    let equations: Vec<Equation> = parser::read("data/day7.txt").unwrap();
    let result = total_calibrations_with_concat(&equations);
    println!("Result: {}", result);
    0
}

fn total_calibrations(equations: &Vec<Equation>) -> BigInt {
    equations
        .iter()
        .filter(|e| e.can_be_evaluated_with_add_and_mult())
        .map(|e| &e.test_value)
        .sum()
}

fn total_calibrations_with_concat(equations: &Vec<Equation>) -> BigInt {
    equations
        .iter()
        .filter(|e| e.can_be_evaluated_with_all_ops())
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
    fn sample_input_part_2() {
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

        let result = total_calibrations_with_concat(&equations);

        assert_eq!(result, 11387.into())
    }
}
