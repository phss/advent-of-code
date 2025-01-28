use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day14.txt").unwrap();
    let mut map = parse(&lines);
    tilt_north(&mut map);
    calculate_load(&map)
}

pub fn part2() -> usize {
    0
}

fn tilt_north(map: &mut Vec<Vec<char>>) {
    let width = map[0].len();
    let height = map.len();

    for x in 0..width {
        let mut free_space_y = 0;

        for y in 0..height {
            match map[y][x] {
                'O' => {
                    (map[free_space_y][x], map[y][x]) = (map[y][x], map[free_space_y][x]);
                    free_space_y += 1;
                }
                '#' => free_space_y = y + 1,
                _ => {}
            }
        }
    }
}

fn calculate_load(map: &Vec<Vec<char>>) -> usize {
    let width = map[0].len();
    let height = map.len();
    let mut load = 0;

    for x in 0..width {
        for y in 0..height {
            match map[y][x] {
                'O' => {
                    load += height - y;
                }
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
        let mut map = parse(&lines);

        tilt_north(&mut map);
        let result = calculate_load(&map);

        assert_eq!(result, 136);
    }

    #[test]
    fn sample_input_part_2() {}
}
