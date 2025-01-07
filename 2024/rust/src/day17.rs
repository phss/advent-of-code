mod ops;

type Registers = (usize, usize, usize);

pub fn part1() -> u32 {
    0
}

pub fn part2() -> u32 {
    0
}

fn interpret(program: &Vec<usize>, registers: &mut Registers) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let combo_operand = program[instruction_pointer + 1];
        instruction_pointer += 2;

        match opcode {
            2 => registers.1 = ops::mod8(value_of(combo_operand, &registers)),
            5 => output.push(ops::mod8(value_of(combo_operand, &registers))),
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
    }

    #[test]
    fn sample_input_part_2() {}
}
