mod keypad;

pub fn part1() -> u32 {
    let codes = vec!["208A", "540A", "685A", "879A", "826A"];
    let codes: Vec<String> = codes.into_iter().map(|s| s.to_string()).collect();
    complexity_sum(&codes) as u32
}

pub fn part2() -> u32 {
    0
}

fn complexity_sum(codes: &Vec<String>) -> usize {
    codes
        .iter()
        .map(|code| {
            let instructions = shortest_instructions(&code);
            let instruction = instructions.first().unwrap();
            let length = instruction.len();
            println!("{code}: {instruction} ({length})");
            let number: usize = code[0..3].parse().unwrap();
            length * number
        })
        .sum()
}

fn shortest_instructions(code: &String) -> Vec<String> {
    let num_keypad = keypad::Keypad::numeric();
    let dir_keypad = keypad::Keypad::directional();
    let steps = vec![&num_keypad, &dir_keypad, &dir_keypad];

    let mut instructions = vec![code.clone()];

    for keypad in steps {
        let new_instructions: Vec<String> = instructions
            .iter()
            .flat_map(|instruction| keypad.shortest_instructions(instruction))
            .collect();

        let min_length = new_instructions.iter().map(String::len).min().unwrap();
        instructions = new_instructions
            .into_iter()
            .filter(|s| s.len() == min_length)
            .collect();
    }

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let codes = vec!["029A", "980A", "179A", "456A", "379A"];
        let codes: Vec<String> = codes.into_iter().map(|s| s.to_string()).collect();

        let result = complexity_sum(&codes);

        assert_eq!(result, 126384);
    }

    #[test]
    fn sample_input_part_2() {}
}
