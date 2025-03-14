use std::ops::BitXor;

use itertools::Itertools;

type Registers = (usize, usize, usize);

pub fn part1() -> u32 {
    let mut registers = (24847151, 0, 0);
    let program = vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 0, 5, 5, 3, 0];
    let result = interpret(&program, &mut registers);
    println!("Result: {:?}", result.iter().join(","));
    0
}

pub fn part2() -> u32 {
    let program = vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 0, 5, 5, 3, 0];
    let a_value = find_lowest_a(&program);
    println!("Result: {:?}", a_value);
    0
}

fn find_lowest_a(program: &Vec<usize>) -> usize {
    let mut a_values = vec![0];

    for digit in program.iter().rev() {
        let mut new_avalues = vec![];

        while let Some(a_value) = a_values.pop() {
            for candidate in 0..8 {
                let a_value = a_value * 8 + candidate;
                let output = interpret(&program, &mut (a_value, 0, 0));
                if output.first().unwrap() == digit {
                    new_avalues.push(a_value);
                }
            }
        }

        a_values = new_avalues.clone();
    }

    a_values.sort();
    *a_values.first().unwrap()
}

fn interpret(program: &Vec<usize>, registers: &mut Registers) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        instruction_pointer += 2;

        match opcode {
            0 => registers.0 = div(registers.0, value_of(operand, &registers)),
            1 => registers.1 = bitwise_xor(registers.1, operand),
            2 => registers.1 = mod8(value_of(operand, &registers)),
            3 => {
                if registers.0 != 0 {
                    instruction_pointer = operand
                }
            }
            4 => registers.1 = bitwise_xor(registers.1, registers.2),
            5 => output.push(mod8(value_of(operand, &registers))),
            6 => registers.1 = div(registers.0, value_of(operand, &registers)),
            7 => registers.2 = div(registers.0, value_of(operand, &registers)),
            _ => panic!("Opcode {opcode} not supported"),
        }
    }

    output
}

fn value_of(combo_operand: usize, registers: &Registers) -> usize {
    match combo_operand {
        value @ 0..=3 => value,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!("Combo operand {combo_operand} not supported"),
    }
}

pub fn div(numerator: usize, value: usize) -> usize {
    let denominator = 2_usize.pow(value as u32);
    numerator / denominator
}

pub fn bitwise_xor(a: usize, b: usize) -> usize {
    a.bitxor(b)
}

pub fn mod8(value: usize) -> usize {
    value.rem_euclid(8)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn sample_input_part_1() {
        let mut registers = (729, 0, 0);
        let program = vec![0, 1, 5, 4, 3, 0];

        let result = interpret(&program, &mut registers).iter().join(",");

        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_1_examples() {
        // If register C contains 9, the program 2,6 would set register B to 1
        let mut registers = (0, 0, 9);
        let _ = interpret(&vec![2, 6], &mut registers);
        assert_eq!(registers.1, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        registers = (10, 0, 0);
        let output = interpret(&vec![5, 0, 5, 1, 5, 4], &mut registers);
        assert_eq!(output, vec![0, 1, 2]);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        registers = (2024, 0, 0);
        let output = interpret(&vec![0, 1, 5, 4, 3, 0], &mut registers);
        assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(registers.0, 0);

        // If register B contains 29, the program 1,7 would set register B to 26.
        registers = (0, 29, 0);
        let _ = interpret(&vec![1, 7], &mut registers);
        assert_eq!(registers.1, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        registers = (0, 2024, 43690);
        let _ = interpret(&vec![4, 0], &mut registers);
        assert_eq!(registers.1, 44354);
    }

    #[test]
    fn sample_input_part_2() {
        let program = vec![0, 3, 5, 4, 3, 0];

        let result = find_lowest_a(&program);

        assert_eq!(result, 117440);
    }
}
