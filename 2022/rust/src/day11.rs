use std::collections::VecDeque;

use crate::parser;

#[derive(Debug, Clone)]
struct Monkey {
    items_worry: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test_divisibility: u64,
    if_true_monkey: u32,
    if_false_monkey: u32,
}

pub fn part1() -> u32 {
    let mut monkeys: Vec<Monkey> = vec![
        Monkey {
            items_worry: VecDeque::from(vec![93, 54, 69, 66, 71]),
            operation: |old| old * 3,
            test_divisibility: 7,
            if_true_monkey: 7,
            if_false_monkey: 1,
        },
        Monkey {
            items_worry: VecDeque::from(vec![89, 51, 80, 66]),
            operation: |old| old * 17,
            test_divisibility: 19,
            if_true_monkey: 5,
            if_false_monkey: 7,
        },
        Monkey {
            items_worry: VecDeque::from(vec![90, 92, 63, 91, 96, 63, 64]),
            operation: |old| old + 1,
            test_divisibility: 13,
            if_true_monkey: 4,
            if_false_monkey: 3,
        },
        Monkey {
            items_worry: VecDeque::from(vec![65, 77]),
            operation: |old| old + 2,
            test_divisibility: 3,
            if_true_monkey: 4,
            if_false_monkey: 6,
        },
        Monkey {
            items_worry: VecDeque::from(vec![76, 68, 94]),
            operation: |old| old * old,
            test_divisibility: 2,
            if_true_monkey: 0,
            if_false_monkey: 6,
        },
        Monkey {
            items_worry: VecDeque::from(vec![86, 65, 66, 97, 73, 83]),
            operation: |old| old + 8,
            test_divisibility: 11,
            if_true_monkey: 2,
            if_false_monkey: 3,
        },
        Monkey {
            items_worry: VecDeque::from(vec![78]),
            operation: |old| old + 6,
            test_divisibility: 17,
            if_true_monkey: 0,
            if_false_monkey: 1,
        },
        Monkey {
            items_worry: VecDeque::from(vec![89, 57, 59, 61, 87, 55, 55, 88]),
            operation: |old| old + 7,
            test_divisibility: 5,
            if_true_monkey: 2,
            if_false_monkey: 5,
        },
    ];

    monkey_business(&mut monkeys)
}

pub fn part2() -> u32 {
    let mut monkeys: Vec<Monkey> = vec![
        Monkey {
            items_worry: VecDeque::from(vec![93, 54, 69, 66, 71]),
            operation: |old| old * 3,
            test_divisibility: 7,
            if_true_monkey: 7,
            if_false_monkey: 1,
        },
        Monkey {
            items_worry: VecDeque::from(vec![89, 51, 80, 66]),
            operation: |old| old * 17,
            test_divisibility: 19,
            if_true_monkey: 5,
            if_false_monkey: 7,
        },
        Monkey {
            items_worry: VecDeque::from(vec![90, 92, 63, 91, 96, 63, 64]),
            operation: |old| old + 1,
            test_divisibility: 13,
            if_true_monkey: 4,
            if_false_monkey: 3,
        },
        Monkey {
            items_worry: VecDeque::from(vec![65, 77]),
            operation: |old| old + 2,
            test_divisibility: 3,
            if_true_monkey: 4,
            if_false_monkey: 6,
        },
        Monkey {
            items_worry: VecDeque::from(vec![76, 68, 94]),
            operation: |old| old * old,
            test_divisibility: 2,
            if_true_monkey: 0,
            if_false_monkey: 6,
        },
        Monkey {
            items_worry: VecDeque::from(vec![86, 65, 66, 97, 73, 83]),
            operation: |old| old + 8,
            test_divisibility: 11,
            if_true_monkey: 2,
            if_false_monkey: 3,
        },
        Monkey {
            items_worry: VecDeque::from(vec![78]),
            operation: |old| old + 6,
            test_divisibility: 17,
            if_true_monkey: 0,
            if_false_monkey: 1,
        },
        Monkey {
            items_worry: VecDeque::from(vec![89, 57, 59, 61, 87, 55, 55, 88]),
            operation: |old| old + 7,
            test_divisibility: 5,
            if_true_monkey: 2,
            if_false_monkey: 5,
        },
    ];

    monkey_business_ten_thousand(&mut monkeys)
}

fn monkey_business(monkeys: &mut Vec<Monkey>) -> u32 {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for mi in 0..monkeys.len() {
            while let Some(item_worry) = monkeys[mi].items_worry.pop_front() {
                inspections[mi] += 1;

                let mut new_worry = (monkeys[mi].operation)(item_worry);
                new_worry /= 3;
                let throw_index = if new_worry % monkeys[mi].test_divisibility == 0 {
                    monkeys[mi].if_true_monkey
                } else {
                    monkeys[mi].if_false_monkey
                };
                monkeys[throw_index as usize].items_worry.push_back(new_worry);
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn monkey_business_ten_thousand(monkeys: &mut Vec<Monkey>) -> u32 {
    let mut inspections = vec![0; monkeys.len()];

    let worry_overflow: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test_divisibility)
        .product();

    for _ in 0..10000 {
        for mi in 0..monkeys.len() {
            while let Some(item_worry) = monkeys[mi].items_worry.pop_front() {
                inspections[mi] += 1;

                let mut new_worry = (monkeys[mi].operation)(item_worry);
                new_worry %= worry_overflow;
                let throw_index = if new_worry % monkeys[mi].test_divisibility == 0 {
                    monkeys[mi].if_true_monkey
                } else {
                    monkeys[mi].if_false_monkey
                };
                monkeys[throw_index as usize].items_worry.push_back(new_worry);
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn sample_input_part_1() {
        let mut monkeys: Vec<Monkey> = vec![
            Monkey {
                items_worry: VecDeque::from(vec![79, 98]),
                operation: |old| old * 19,
                test_divisibility: 23,
                if_true_monkey: 2,
                if_false_monkey: 3,
            },
            Monkey {
                items_worry: VecDeque::from(vec![54, 65, 75, 74]),
                operation: |old| old + 6,
                test_divisibility: 19,
                if_true_monkey: 2,
                if_false_monkey: 0,
            },
            Monkey {
                items_worry: VecDeque::from(vec![79, 60, 97]),
                operation: |old| old * old,
                test_divisibility: 13,
                if_true_monkey: 1,
                if_false_monkey: 3,
            },
            Monkey {
                items_worry: VecDeque::from(vec![74]),
                operation: |old| old + 3,
                test_divisibility: 17,
                if_true_monkey: 0,
                if_false_monkey: 1,
            },
        ];

        assert_eq!(monkey_business(&mut monkeys), 10605);
    }

    #[test]
    fn sample_input_part_2() {
        let mut monkeys: Vec<Monkey> = vec![
            Monkey {
                items_worry: VecDeque::from(vec![79, 98]),
                operation: |old| old * 19,
                test_divisibility: 23,
                if_true_monkey: 2,
                if_false_monkey: 3,
            },
            Monkey {
                items_worry: VecDeque::from(vec![54, 65, 75, 74]),
                operation: |old| old + 6,
                test_divisibility: 19,
                if_true_monkey: 2,
                if_false_monkey: 0,
            },
            Monkey {
                items_worry: VecDeque::from(vec![79, 60, 97]),
                operation: |old| old * old,
                test_divisibility: 13,
                if_true_monkey: 1,
                if_false_monkey: 3,
            },
            Monkey {
                items_worry: VecDeque::from(vec![74]),
                operation: |old| old + 3,
                test_divisibility: 17,
                if_true_monkey: 0,
                if_false_monkey: 1,
            },
        ];

        assert_eq!(monkey_business_ten_thousand(&mut monkeys), 2713310158);
    }
}
