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
    machines.iter().map(cheapest_win).sum()
}

fn cheapest_win(machine: &Machine) -> u32 {
    let mut cheapeast = 100000;
    (0..=100).for_each(|x| {
        (0..=100).for_each(|y| {
            let a = machine.button_a.0 * x + machine.button_b.0 * y;
            let b = machine.button_a.1 * x + machine.button_b.1 * y;
            let c = (x * 3) + y;

            if a == machine.prize.0 && b == machine.prize.1 && c < cheapeast {
                cheapeast = c;
            }
        })
    });

    if cheapeast == 100000 {
        0
    } else {
        cheapeast as u32
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

        assert_eq!(result, 480);
    }

    #[test]
    fn sample_input_part_2() {}
}
