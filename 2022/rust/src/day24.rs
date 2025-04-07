use std::collections::HashMap;

use crate::parser;

type position = (usize, usize);

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day24.txt").unwrap();
    let (start, end, blizzards) = parse(lines);
    min_steps(start, end, blizzards)
}

pub fn part2() -> usize {
    0
}

fn min_steps(
    start: (usize, usize),
    end: (usize, usize),
    blizzards: HashMap<position, Vec<char>>,
) -> usize {
    0
}

fn parse(lines: Vec<String>) -> (position, position, HashMap<position, Vec<char>>) {
    let start = (1, 0);
    let end = (lines.len() - 1, lines[0].len() - 2);
    let mut blizzards = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell != '.' && cell != '#' {
                blizzards.entry((x, y)).or_insert_with(Vec::new).push(cell);
            }
        }
    }

    (start, end, blizzards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
        ];
        let lines: Vec<String> = input.iter().map(|s| s.parse().unwrap()).collect();
        let (start, end, blizzards) = parse(lines);

        let result = min_steps(start, end, blizzards);

        assert_eq!(result, 18);
    }

    #[test]
    fn sample_input_part_2() {}
}
