use std::usize;

use num_bigint::BigInt;
use regex::Regex;

use crate::parser;

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    let machines = parse(&lines);
    fewest_tokens_win(&machines)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    let machines = parse(&lines)
        .into_iter()
        .map(|machine| Machine {
            button_a: machine.button_a,
            button_b: machine.button_b,
            prize: (
                machine.prize.0 + 10000000000000,
                machine.prize.1 + 10000000000000,
            ),
        })
        .collect();
    fewest_tokens_win(&machines)
}

fn parse(lines: &Vec<String>) -> Vec<Machine> {
    let button_regexp = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regexp = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let parse_line = |line: &String, regex: &Regex| -> (usize, usize) {
        regex
            .captures(line)
            .map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap()))
            .unwrap()
    };

    lines
        .split(String::is_empty)
        .map(|machine_lines| Machine {
            button_a: parse_line(&machine_lines[0], &button_regexp),
            button_b: parse_line(&machine_lines[1], &button_regexp),
            prize: parse_line(&machine_lines[2], &prize_regexp),
        })
        .collect()
}

fn fewest_tokens_win(machines: &Vec<Machine>) -> u32 {
    let result = machines.iter().map(cheapest_win).reduce(|a, b| a + b);
    println!("{:?}", result);
    0
}

fn cheapest_win(machine: &Machine) -> BigInt {
    // Equations where:
    // - x: number of times button A is pressed
    // - y: number of times button b is pressed
    // a * x + b * y = u
    // c * x + d * y = v
    let (a, c): (BigInt, BigInt) = (machine.button_a.0.into(), machine.button_a.1.into());
    let (b, d): (BigInt, BigInt) = (machine.button_b.0.into(), machine.button_b.1.into());
    let (u, v): (BigInt, BigInt) = (machine.prize.0.into(), machine.prize.1.into());

    let x = (&u * &d - &v * &b) / (&a * &d - &c * &b);
    let x_is_int = (&u * &d - &v * &b) % (&a * &d - &c * &b) == 0.into();
    let y = (&u - &a * &x) / &b;
    let y_is_int = (&u - &a * &x) % &b == 0.into();

    if x_is_int && y_is_int {
        3 * x + y
    } else {
        0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let machines = vec![
            Machine {
                button_a: (94, 34),
                button_b: (22, 67),
                prize: (8400, 5400),
            },
            Machine {
                button_a: (26, 66),
                button_b: (67, 21),
                prize: (12748, 12176),
            },
            Machine {
                button_a: (17, 86),
                button_b: (84, 37),
                prize: (7870, 6450),
            },
            Machine {
                button_a: (69, 23),
                button_b: (27, 71),
                prize: (18641, 10278),
            },
        ];

        let result = fewest_tokens_win(&machines);

        assert_eq!(result, 0);
    }

    #[test]
    fn sample_input_part_2() {}
}
