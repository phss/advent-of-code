use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor},
};

use itertools::Itertools;

use crate::parser;

type Operation = (String, String, String, String);

struct Simulator {
    inputs: HashMap<String, u8>,
    operations: HashMap<String, Operation>,
}

impl Simulator {
    fn parse(lines: &Vec<String>) -> Self {
        let mut parts = lines.split(|line| line.is_empty());
        let mut inputs = HashMap::new();
        let mut operations = HashMap::new();
        let op_regex = regex::Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();

        for s in parts.next().unwrap() {
            let parts: Vec<&str> = s.split(": ").collect();
            let wire = parts[0].to_string();
            let value = parts[1].parse().unwrap();
            inputs.insert(wire, value);
        }

        for s in parts.next().unwrap() {
            if let Some(captures) = op_regex.captures(s) {
                let wire_a = captures[1].to_string();
                let op = captures[2].to_string();
                let wire_b = captures[3].to_string();
                let output = captures[4].to_string();
                operations.insert(output.clone(), (op, wire_a, wire_b, output));
            }
        }

        Self { inputs, operations }
    }

    fn simulate(&self, output_bits: u32) -> usize {
        let mut output = String::new();
        for bit in (0..output_bits).rev() {
            let output_wire = format!("z{:02}", bit);
            let output_value = self.value_of(&output_wire).to_string();
            output.push_str(&output_value);
        }
        usize::from_str_radix(&output, 2).unwrap()
    }

    fn value_of(&self, wire: &str) -> u8 {
        if let Some(value) = self.inputs.get(wire) {
            return *value;
        }

        let (op, wire_a, wire_b, _) = self.operations.get(wire).unwrap();
        let value_a = self.value_of(&wire_a);
        let value_b = self.value_of(&wire_b);

        match op.as_str() {
            "AND" => value_a.bitand(value_b),
            "OR" => value_a.bitor(value_b),
            "XOR" => value_a.bitxor(value_b),
            _ => panic!("not supported"),
        }
    }

    // val wrong4 = gates.values.filter { gate ->
    //     if ((gate.lhs.contains('x') || gate.rhs.contains('x') || gate.lhs.contains('y') || gate.rhs.contains('y')) && gate.op == "AND") {
    //         gates.values.none { (it.lhs == gate.res || it.rhs == gate.res) && it.op == "OR" }
    //     } else false
    // }.filter { !it.lhs.contains("00") }

    fn detect_wrong_gates(&self) -> Vec<String> {
        let a: Vec<String> = self
            .operations
            .values()
            .filter(|(op, _, _, output)| op != "XOR" && output.starts_with("z") && output != "z45")
            .map(|(_, _, _, output)| output.clone())
            .collect();

        let b: Vec<String> = self
            .operations
            .values()
            .filter(|(op, a, b, output)| {
                op == "XOR"
                    && !output.starts_with("z")
                    && !a.starts_with("x")
                    && !a.starts_with("y")
                    && !b.starts_with("x")
                    && !b.starts_with("y")
            })
            .map(|(_, _, _, output)| output.clone())
            .collect();

        let c: Vec<String> = self
            .operations
            .values()
            .filter(|(op, a, b, output)| {
                if op == "XOR"
                    && !a.ends_with("00")
                    && (a.starts_with("x")
                        || a.starts_with("y")
                        || b.starts_with("x")
                        || b.starts_with("y"))
                {
                    !self
                        .operations
                        .values()
                        .any(|(other_op, other_a, other_b, _)| {
                            other_op == "XOR" && (other_a == output || other_b == output)
                        })
                } else {
                    false
                }
            })
            .map(|(_, _, _, output)| output.clone())
            .collect();

        let d: Vec<String> = self
            .operations
            .values()
            .filter(|(op, a, b, output)| {
                if op == "AND"
                    && !a.ends_with("00")
                    && (a.starts_with("x")
                        || a.starts_with("y")
                        || b.starts_with("x")
                        || b.starts_with("y"))
                {
                    !self
                        .operations
                        .values()
                        .any(|(other_op, other_a, other_b, _)| {
                            other_op == "OR" && (other_a == output || other_b == output)
                        })
                } else {
                    false
                }
            })
            .map(|(_, _, _, output)| output.clone())
            .collect();

        let mut result = Vec::new();
        result.extend(a);
        result.extend(b);
        result.extend(c);
        result.extend(d);
        result
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day24.txt").unwrap();
    let simulator = Simulator::parse(&lines);
    let result = simulator.simulate(46);
    println!("Result {result}");
    0
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day24.txt").unwrap();
    let simulator = Simulator::parse(&lines);
    let result = simulator.detect_wrong_gates().iter().sorted().join(",");
    println!("Result {result}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "x00: 1",
            "x01: 0",
            "x02: 1",
            "x03: 1",
            "x04: 0",
            "y00: 1",
            "y01: 1",
            "y02: 1",
            "y03: 1",
            "y04: 1",
            "",
            "ntg XOR fgs -> mjb",
            "y02 OR x01 -> tnw",
            "kwq OR kpj -> z05",
            "x00 OR x03 -> fst",
            "tgd XOR rvg -> z01",
            "vdt OR tnw -> bfw",
            "bfw AND frj -> z10",
            "ffh OR nrd -> bqk",
            "y00 AND y03 -> djm",
            "y03 OR y00 -> psh",
            "bqk OR frj -> z08",
            "tnw OR fst -> frj",
            "gnj AND tgd -> z11",
            "bfw XOR mjb -> z00",
            "x03 OR x00 -> vdt",
            "gnj AND wpb -> z02",
            "x04 AND y00 -> kjc",
            "djm OR pbm -> qhw",
            "nrd AND vdt -> hwm",
            "kjc AND fst -> rvg",
            "y04 OR y02 -> fgs",
            "y01 AND x02 -> pbm",
            "ntg OR kjc -> kwq",
            "psh XOR fgs -> tgd",
            "qhw XOR tgd -> z09",
            "pbm OR djm -> kpj",
            "x03 XOR y03 -> ffh",
            "x00 XOR y04 -> ntg",
            "bfw OR bqk -> z06",
            "nrd XOR fgs -> wpb",
            "frj XOR qhw -> z04",
            "bqk OR frj -> z07",
            "y03 OR x01 -> nrd",
            "hwm AND bqk -> z03",
            "tgd XOR rvg -> z12",
            "tnw OR pbm -> gnj",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();

        let simulator = Simulator::parse(&lines);
        let result = simulator.simulate(13);

        assert_eq!(result, 2024)
    }

    #[test]
    fn sample_input_part_1_small_example() {
        let lines = vec![
            "x00: 1",
            "x01: 1",
            "x02: 1",
            "y00: 0",
            "y01: 1",
            "y02: 0",
            "",
            "x00 AND y00 -> z00",
            "x01 XOR y01 -> z01",
            "x02 OR y02 -> z02",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();

        let simulator = Simulator::parse(&lines);
        let result = simulator.simulate(3);

        assert_eq!(result, 4)
    }

    #[test]
    fn sample_input_part_2() {}
}
