use itertools::Itertools;

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    let (map, moves, initial_position) = parse(&lines);
    sum_of_gps_coords(&map, &moves, initial_position)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    0
}
fn sum_of_gps_coords(
    map: &Vec<Vec<char>>,
    moves: &Vec<char>,
    initial_position: (usize, usize),
) -> u32 {
    0
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
    let mut raw = lines.split(|line| line.is_empty());

    let map: Vec<Vec<char>> = raw
        .next()
        .unwrap()
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let moves = raw.next().unwrap().iter().flat_map(|s| s.chars()).collect();

    let mut initial_position = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '@') {
            initial_position = (x, y);
            break;
        }
    }

    (map, moves, initial_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#..O@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
            "",
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (map, moves, initial_position) = parse(&lines);

        let result = sum_of_gps_coords(&map, &moves, initial_position);

        assert_eq!(result, 10092);
    }

    #[test]
    fn sample_input_part_2() {}
}
