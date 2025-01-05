mod map;
use std::collections::HashSet;

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    let (mut map, moves) = map::parse(&lines);
    sum_of_gps_coords(&mut map, &moves)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    let (map, moves) = map::parse(&lines);
    let mut wide_map = map::widen(&map);
    sum_of_gps_coords(&mut wide_map, &moves)
}

fn sum_of_gps_coords(map: &mut Vec<Vec<char>>, moves: &Vec<char>) -> u32 {
    let (mut x, mut y) = map::robot_position(map);

    moves.iter().map(to_dir).for_each(|dir @ (dir_x, dir_y)| {
        if push(map, &vec![(x, y)], dir) {
            x = x.wrapping_add_signed(dir_x);
            y = y.wrapping_add_signed(dir_y);
        }
    });

    map::print(map);
    map::score(map)
}

fn to_dir(m: &char) -> (isize, isize) {
    match m {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => (0, 0),
    }
}

fn push(
    map: &mut Vec<Vec<char>>,
    to_push: &Vec<(usize, usize)>,
    dir @ (dir_x, dir_y): (isize, isize),
) -> bool {
    if to_push.is_empty() {
        return true;
    }

    let vertical_move = dir_y != 0;
    let mut next_level = HashSet::new();

    for (x, y) in to_push {
        let next_x = x.wrapping_add_signed(dir_x);
        let next_y = y.wrapping_add_signed(dir_y);
        let c = map[next_y][next_x];

        match c {
            '#' => return false,
            '.' => {}
            _ => {
                next_level.insert((next_x, next_y));
            }
        }

        if vertical_move && (c == '[' || c == ']') {
            let next_x = if c == '[' { next_x + 1 } else { next_x - 1 };
            next_level.insert((next_x, next_y));
        }
    }

    let can_push = push(map, &next_level.into_iter().collect(), dir);

    if can_push {
        for (x, y) in to_push {
            let next_x = x.wrapping_add_signed(dir_x);
            let next_y = y.wrapping_add_signed(dir_y);
            map[next_y][next_x] = map[*y][*x];
            map[*y][*x] = '.';
        }
    }

    can_push
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
        let (mut map, moves) = map::parse(&lines);

        let result = sum_of_gps_coords(&mut map, &moves);

        assert_eq!(result, 10092);
    }

    #[test]
    fn sample_input_part_1_smaller_example() {
        let lines = vec![
            "########",
            "#..O.O.#",
            "##@.O..#",
            "#...O..#",
            "#.#.O..#",
            "#...O..#",
            "#......#",
            "########",
            "",
            "<^^>>>vv<v>>v<<",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (mut map, moves) = map::parse(&lines);

        let result = sum_of_gps_coords(&mut map, &moves);

        assert_eq!(result, 2028);
    }

    #[test]
    fn sample_input_part_2() {
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
        let (map, moves) = map::parse(&lines);
        let mut wide_map = map::widen(&map);

        let result = sum_of_gps_coords(&mut wide_map, &moves);

        assert_eq!(result, 9021);
    }

    #[test]
    fn sample_input_part_2_smaller_example() {
        let lines = vec![
            "#######",
            "#...#.#",
            "#.....#",
            "#..OO@#",
            "#..O..#",
            "#.....#",
            "#######",
            "",
            "<vv<<^^<<^^",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (map, moves) = map::parse(&lines);
        let mut wide_map = map::widen(&map);

        let result = sum_of_gps_coords(&mut wide_map, &moves);

        assert_eq!(result, 618);
    }
}
