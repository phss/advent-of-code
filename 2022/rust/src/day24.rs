use std::collections::HashMap;

use crate::parser;

type position = (usize, usize);

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day24.txt").unwrap();
    let (dimensions, blizzards) = parse(lines);
    min_steps(dimensions, blizzards)
}

pub fn part2() -> usize {
    0
}

fn min_steps((width, height): (usize, usize), blizzards: HashMap<position, Vec<char>>) -> usize {
    let start = (1, 0);
    let end = (width - 2, height - 1);

    0
}

fn parse(lines: Vec<String>) -> (position, HashMap<position, Vec<char>>) {
    let width = lines[0].len();
    let height = lines.len();
    let mut blizzards = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell != '.' && cell != '#' {
                blizzards.entry((x, y)).or_insert_with(Vec::new).push(cell);
            }
        }
    }

    ((width, height), blizzards)
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
        let (dimensions, blizzards) = parse(lines);

        let result = min_steps(dimensions, blizzards);

        assert_eq!(result, 18);
    }

    #[test]
    fn sample_input_part_2() {}
}
