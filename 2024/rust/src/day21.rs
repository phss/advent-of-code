mod keypad;

pub fn part1() -> u32 {
    let codes = vec!["208A", "540A", "685A", "879A", "826A"];
    let codes: Vec<String> = codes.into_iter().map(|s| s.to_string()).collect();
    complexity_sum(&codes, 2) as u32
}

pub fn part2() -> u32 {
    let codes = vec!["208A", "540A", "685A", "879A", "826A"];
    let codes: Vec<String> = codes.into_iter().map(|s| s.to_string()).collect();
    let result = complexity_sum(&codes, 25);
    println!("Result {result}");
    0
}

fn complexity_sum(codes: &Vec<String>, depth: usize) -> usize {
    let num_keypad = keypad::Keypad::numeric();
    let mut dir_keypad = keypad::Keypad::directional();

    codes
        .iter()
        .map(|code| {
            let length = shortest_instructions(&code, depth, &num_keypad, &mut dir_keypad);
            let number: usize = code[0..3].parse().unwrap();
            length * number
        })
        .sum()
}

fn shortest_instructions(
    code: &String,
    depth: usize,
    num_keypad: &keypad::Keypad,
    dir_keypad: &mut keypad::Keypad,
) -> usize {
    num_keypad
        .shortest_instructions(code)
        .iter()
        .map(|i| dir_keypad.minimum_length_of(i, depth))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let codes = vec!["029A", "980A", "179A", "456A", "379A"];

        let codes: Vec<String> = codes.into_iter().map(|s| s.to_string()).collect();

        let result = complexity_sum(&codes, 2);

        assert_eq!(result, 126384);
    }

    #[test]
    fn sample_input_part_2() {}
}
