use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day14.txt").unwrap();
    let map = parse(&lines);
    calculate_load(&map)
}

pub fn part2() -> usize {
    0
}

fn calculate_load(map: &Vec<Vec<char>>) -> usize {
    let width = map[0].len();
    let height = map.len();
    let mut load = 0;

    for x in 0..width {
        let mut col_load = height;

        for y in 0..height {
            match map[y][x] {
                'O' => {
                    load += col_load;
                    col_load -= 1;
                }
                '#' => col_load = height - y - 1,
                _ => {}
            }
        }
    }

    load
}

pub fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = calculate_load(&map);

        assert_eq!(result, 136);
    }

    #[test]
    fn sample_input_part_2() {}
}
