use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Keypad {
    inputs_map: HashMap<char, HashMap<char, Vec<String>>>,
    cache: HashMap<(String, usize), usize>,
}

impl Keypad {
    pub fn numeric() -> Self {
        Self::new(&vec![
            "789".chars().collect(),
            "456".chars().collect(),
            "123".chars().collect(),
            " 0A".chars().collect(),
        ])
    }

    pub fn directional() -> Self {
        Self::new(&vec![" ^A".chars().collect(), "<v>".chars().collect()])
    }

    fn new(raw: &Vec<Vec<char>>) -> Self {
        let mut inputs_map = HashMap::new();

        for (y, row) in raw.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c != ' ' {
                    inputs_map.insert(*c, Keypad::inputs_from(&raw, (x, y)));
                }
            }
        }

        Self {
            inputs_map,
            cache: HashMap::new(),
        }
    }

    fn inputs_from(raw: &Vec<Vec<char>>, start: (usize, usize)) -> HashMap<char, Vec<String>> {
        let mut inputs = HashMap::new();
        let width = raw[0].len() - 1;
        let height = raw.len() - 1;

        let directions = [(1, 0, ">"), (-1, 0, "<"), (0, 1, "v"), (0, -1, "^")];
        let mut paths = VecDeque::new();
        paths.push_back((
            start,
            "".to_string(),
            vec![start].into_iter().collect::<HashSet<_>>(),
        ));
        while let Some(((x, y), input, visited)) = paths.pop_front() {
            inputs
                .entry(raw[y][x])
                .or_insert(Vec::new())
                .push(input.to_string());

            for (dir_x, dir_y, dir_char) in directions {
                let next_x = x.checked_add_signed(dir_x).unwrap_or(0).min(width);
                let next_y = y.checked_add_signed(dir_y).unwrap_or(0).min(height);
                let next_position = (next_x, next_y);

                if raw[next_y][next_x] == ' ' || visited.contains(&next_position) {
                    continue;
                }

                let mut visited = visited.clone();
                visited.insert(next_position);

                paths.push_back((next_position, input.clone() + dir_char, visited));
            }
        }

        let keys: Vec<_> = inputs.keys().cloned().collect();
        for end in keys {
            inputs.entry(end).and_modify(|input| {
                let smallest = input.iter().map(String::len).min().unwrap();
                *input = input
                    .iter()
                    .filter(|s| s.len() == smallest)
                    .cloned()
                    .collect();
            });
        }

        inputs
    }

    pub fn shortest_instructions(&self, code: &String) -> Vec<String> {
        let mut instructions = vec!["".to_string()];
        let mut from = 'A';

        for to in code.chars() {
            let inputs = self.inputs(&from, &to);

            let mut new_instructions = Vec::new();
            for instruction in &instructions {
                for input in inputs {
                    let mut new_instruction = instruction.clone();
                    new_instruction.push_str(input);
                    new_instruction.push('A');
                    new_instructions.push(new_instruction);
                }
            }

            instructions = new_instructions;
            from = to;
        }

        let smallest = instructions.iter().map(String::len).min().unwrap();
        instructions
            .iter()
            .filter(|s| s.len() == smallest)
            .cloned()
            .collect()

        // instructions
    }

    fn inputs(&self, from: &char, to: &char) -> &Vec<String> {
        self.inputs_map.get(from).unwrap().get(to).unwrap()
    }

    pub fn minimum_length_of(&mut self, instruction: &String, depth: usize) -> usize {
        if depth == 0 {
            return instruction.len();
        }

        let cache_key = (instruction.clone(), depth);
        if let Some(length) = self.cache.get(&cache_key) {
            return *length;
        }

        let mut length = 0;
        for sub_instruction in instruction.split_inclusive("A") {
            length += self
                .shortest_instructions(&sub_instruction.to_string())
                .iter()
                .map(|i| self.minimum_length_of(i, depth - 1))
                .min()
                .unwrap();
        }

        self.cache.insert(cache_key, length);
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypad_shortest_instructions() {
        let num_keypad = Keypad::numeric();
        let dir_keypad = Keypad::directional();

        let first_robot_instructions = num_keypad.shortest_instructions(&"029A".to_string());
        assert_eq!(
            first_robot_instructions,
            vec!["<A^A>^^AvvvA", "<A^A^>^AvvvA", "<A^A^^>AvvvA"]
        );

        let second_robot_instructions: Vec<String> = first_robot_instructions
            .iter()
            .flat_map(|instruction| dir_keypad.shortest_instructions(instruction))
            .collect();
        assert!(second_robot_instructions.contains(&"v<<A>>^A<A>AvA<^AA>A<vAAA>^A".to_string()));
    }
}
