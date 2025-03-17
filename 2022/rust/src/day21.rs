use std::collections::HashMap;

use crate::parser;

#[derive(Debug)]
enum Monkey {
    Number(usize),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

pub fn part1() -> usize {
    0
}

pub fn part2() -> usize {
    0
}

fn parse(lines: Vec<String>) -> HashMap<String, Monkey> {
    let mut monkeys = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0].to_string();
        let job = parts[1];

        let monkey = if job.contains(" + ") {
            let operands: Vec<&str> = job.split(" + ").collect();
            Monkey::Add(operands[0].to_string(), operands[1].to_string())
        } else if job.contains(" - ") {
            let operands: Vec<&str> = job.split(" - ").collect();
            Monkey::Sub(operands[0].to_string(), operands[1].to_string())
        } else if job.contains(" * ") {
            let operands: Vec<&str> = job.split(" * ").collect();
            Monkey::Mult(operands[0].to_string(), operands[1].to_string())
        } else if job.contains(" / ") {
            let operands: Vec<&str> = job.split(" / ").collect();
            Monkey::Div(operands[0].to_string(), operands[1].to_string())
        } else {
            Monkey::Number(job.parse().unwrap())
        };

        monkeys.insert(name, monkey);
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "root: pppw + sjmn",
            "dbpl: 5",
            "cczh: sllz + lgvd",
            "zczc: 2",
            "ptdq: humn - dvpt",
            "dvpt: 3",
            "lfqf: 4",
            "humn: 5",
            "ljgn: 2",
            "sjmn: drzm * dbpl",
            "sllz: 4",
            "pppw: cczh / lfqf",
            "lgvd: ljgn * ptdq",
            "drzm: hmdt - zczc",
            "hmdt: 32",
        ];
        let lines: Vec<String> = input.iter().map(|s| s.parse().unwrap()).collect();
        let monkeys = parse(lines);

        println!("{:?}", monkeys);

        // let result = root_number(&monkeys);

        // assert_eq!(result, 150);
    }

    #[test]
    fn sample_input_part_2() {}
}
