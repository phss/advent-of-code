use std::collections::HashSet;

use itertools::Itertools;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day16.txt").unwrap();
    let map = parse(&lines);
    count_energized(&map)
}

pub fn part2() -> usize {
    0
}

fn count_energized(map: &Vec<Vec<char>>) -> usize {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let start: ((isize, isize), (isize, isize)) = ((-1, 0), (1, 0));
    let mut visited = HashSet::new();
    let mut search_heap = Vec::new();
    search_heap.push(start);

    while let Some(((x, y), current_direction @ (dir_x, dir_y))) = search_heap.pop() {
        let next_x = x + dir_x;
        let next_y = y + dir_y;
        let next_position = (next_x, next_y);

        if next_x < 0
            || next_x >= width
            || next_y < 0
            || next_y >= height
            || visited.contains(&(next_position, current_direction))
        {
            continue;
        }
        visited.insert((next_position, current_direction));

        match map[next_y as usize][next_x as usize] {
            '-' if dir_y != 0 => {
                search_heap.push((next_position, (1, 0)));
                search_heap.push((next_position, (-1, 0)));
            }
            '|' if dir_x != 0 => {
                search_heap.push((next_position, (0, 1)));
                search_heap.push((next_position, (0, -1)));
            }
            '\\' => search_heap.push((next_position, (dir_y, dir_x))),
            '/' => search_heap.push((next_position, (-dir_y, -dir_x))),
            _ => search_heap.push((next_position, current_direction)),
        }
    }

    visited.iter().map(|(node, _)| node).unique().count()
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            ".|...\\....",
            "|.-.\\.....",
            ".....|-...",
            "........|.",
            "..........",
            ".........\\",
            "..../.\\\\..",
            ".-.-/..|..",
            ".|....-|.\\",
            "..//.|....",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = count_energized(&map);

        assert_eq!(result, 46);
    }

    #[test]
    fn sample_input_part_2() {}
}
