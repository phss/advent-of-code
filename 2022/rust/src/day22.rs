use std::collections::HashMap;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day22.txt").unwrap();
    let (map, adj, instructions) = parse(lines, 150);
    final_password(&map, &adj, &instructions)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day22.txt").unwrap();
    // let (map, adj, instructions) = parse_cube(lines, 150);
    // final_password(&map, &adj, &instructions)
    todo!()
}

fn final_password(
    map: &Vec<Vec<char>>,
    adj: &HashMap<(usize, usize, char), (usize, usize, char)>,
    instructions: &Vec<(usize, char)>,
) -> usize {
    let position = start_position(map);
    let mut node = (position.0, position.1, '>');

    for (moves, turn) in instructions {
        node = move_to(adj, node, *moves);
        node.2 = turn_to(node.2, *turn);
    }

    score(node)
}

fn start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '.' {
                return (x, y);
            }
        }
    }

    (0, 0)
}

fn move_to(
    adj: &HashMap<(usize, usize, char), (usize, usize, char)>,
    start: (usize, usize, char),
    moves: usize,
) -> (usize, usize, char) {
    let mut node = start.clone();

    for _ in 0..moves {
        if let Some(next) = adj.get(&node) {
            node = *next;
        } else {
            break;
        }
    }

    node
}

fn turn_to(direction: char, turn: char) -> char {
    match (direction, turn) {
        ('>', 'L') => '^',
        ('>', 'R') => 'v',
        ('v', 'L') => '>',
        ('v', 'R') => '<',
        ('<', 'L') => 'v',
        ('<', 'R') => '^',
        ('^', 'L') => '<',
        ('^', 'R') => '>',
        (_, ' ') => direction,
        _ => panic!("invalid option"),
    }
}

fn score((x, y, direction): (usize, usize, char)) -> usize {
    let row_score = (y + 1) * 1000;
    let col_score = (x + 1) * 4;
    let dir_score = match direction {
        '>' => 0,
        'v' => 1,
        '<' => 2,
        '^' => 3,
        _ => panic!("invalid option"),
    };

    row_score + col_score + dir_score
}

fn parse(
    lines: Vec<String>,
    width: usize,
) -> (
    Vec<Vec<char>>,
    HashMap<(usize, usize, char), (usize, usize, char)>,
    Vec<(usize, char)>,
) {
    let mut raw = lines.split(|line| line.is_empty());

    let map: Vec<Vec<char>> = raw
        .next()
        .unwrap()
        .iter()
        .map(|s| {
            let mut row: Vec<char> = s.chars().collect();
            row.resize(width, ' ');
            row
        })
        .collect();

    let directions: Vec<(isize, isize, char)> =
        vec![(1, 0, '>'), (-1, 0, '<'), (0, 1, 'v'), (0, -1, '^')];
    let mut adj = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != '.' {
                continue;
            }

            for (dir_x, dir_y, dir_char) in &directions {
                let max_x = map[y].len() - 1;
                let max_y = map.len() - 1;

                let wrap_left = *dir_x < 0 && (x == 0 || map[y][x - 1] == ' ');
                let wrap_right = *dir_x > 0 && (x == max_x || map[y][x + 1] == ' ');
                let wrap_up = *dir_y < 0 && (y == 0 || map[y - 1][x] == ' ');
                let wrap_down = *dir_y > 0 && (y == max_y || map[y + 1][x] == ' ');

                let new_x = if wrap_left {
                    (0..=max_x)
                        .rev()
                        .find(|new_x| map[y][*new_x] != ' ')
                        .unwrap()
                } else if wrap_right {
                    (0..=max_x).find(|new_x| map[y][*new_x] != ' ').unwrap()
                } else {
                    x.wrapping_add_signed(*dir_x)
                };

                let new_y = if wrap_up {
                    (0..=max_y)
                        .rev()
                        .find(|new_y| map[*new_y][x] != ' ')
                        .unwrap()
                } else if wrap_down {
                    (0..=max_y).find(|new_y| map[*new_y][x] != ' ').unwrap()
                } else {
                    y.wrapping_add_signed(*dir_y)
                };

                if map[new_y][new_x] == '#' {
                    continue;
                }

                adj.insert((x, y, *dir_char), (new_x, new_y, *dir_char));
            }
        }
    }

    let direction = |c| c == 'R' || c == 'L';
    let instruction_raw = raw.next().unwrap().first().unwrap();
    let instructions = instruction_raw
        .split_inclusive(direction)
        .map(|elem| {
            let (num, dir) = if elem.ends_with(direction) {
                elem.split_at(elem.len() - 1)
            } else {
                (elem, "")
            };
            (
                num.parse::<usize>().unwrap(),
                dir.chars().next().unwrap_or(' '),
            )
        })
        .collect();

    (map, adj, instructions)
}

fn parse_cube(
    map: &Vec<Vec<char>>,
    adj: &HashMap<(usize, usize, char), (usize, usize, char)>,
    cube_dimension: usize,
    cubes: Vec<(usize, usize)>,
    overrides: Vec<(usize, char, usize, char, bool)>,
) -> HashMap<(usize, usize, char), (usize, usize, char)> {
    let mut cube_adj = adj.clone();
    let max_i = cube_dimension - 1;

    for (from, from_dir, to, to_dir, invert) in overrides {
        let from_cube = cubes[from];
        let to_cube = cubes[to];

        for i in 0..cube_dimension {
            let (from_x, from_y) = match from_dir {
                '^' => (from_cube.0 + i, from_cube.1),
                'v' => (from_cube.0 + i, from_cube.1 + max_i),
                '>' => (from_cube.0 + max_i, from_cube.1 + i),
                '<' => (from_cube.0, from_cube.1 + i),
                _ => panic!("unreacheable"),
            };

            let (to_x, to_y) = match (to_dir, invert) {
                ('^', false) => (to_cube.0 + i, to_cube.1 + max_i),
                ('^', true) => (to_cube.0 + max_i - i, to_cube.1 + max_i),
                ('v', false) => (to_cube.0 + i, to_cube.1),
                ('v', true) => (to_cube.0 + max_i - i, to_cube.1),
                ('>', false) => (to_cube.0, to_cube.1 + i),
                ('>', true) => (to_cube.0, to_cube.1 + max_i - i),
                ('<', false) => (to_cube.0 + max_i, to_cube.1 + i),
                ('<', true) => (to_cube.0 + max_i, to_cube.1 + max_i - i),
                _ => panic!("unreacheable"),
            };

            if map[to_y][to_x] == '.' {
                cube_adj.insert((from_x, from_y, from_dir), (to_x, to_y, to_dir));
            }
        }
    }

    cube_adj
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "        ...#    ",
            "        .#..    ",
            "        #...    ",
            "        ....    ",
            "...#.......#    ",
            "........#...    ",
            "..#....#....    ",
            "..........#.    ",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ];
        let lines: Vec<String> = input.iter().map(|s| s.parse().unwrap()).collect();
        let (map, adj, instructions) = parse(lines, 20);

        let result = final_password(&map, &adj, &instructions);

        assert_eq!(result, 6032);
    }

    #[test]
    fn sample_input_part_2() {
        let input = vec![
            "        ...#    ",
            "        .#..    ",
            "        #...    ",
            "        ....    ",
            "...#.......#    ",
            "........#...    ",
            "..#....#....    ",
            "..........#.    ",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ];
        let lines: Vec<String> = input.iter().map(|s| s.parse().unwrap()).collect();
        let (map, adj, instructions) = parse(lines, 20);
        let cube_adj = parse_cube(
            &map,
            &adj,
            4,
            vec![(8, 0), (0, 4), (4, 4), (8, 4), (8, 8), (12, 8)],
            vec![
                (0, '^', 1, 'v', true),
                (0, '<', 2, 'v', false),
                (0, '>', 5, '<', true),
                (1, '^', 1, 'v', true),
                (1, '<', 5, '^', true),
                (1, 'v', 4, '^', true),
                (2, '^', 0, '>', false),
                (2, 'v', 4, '>', true),
                (3, '>', 5, 'v', true),
                (4, '<', 2, '^', true),
                (4, 'v', 1, '^', true),
                (5, '^', 3, '<', true),
                (5, '>', 0, '<', true),
                (5, 'v', 1, '>', true),
            ],
        );

        let result = final_password(&map, &cube_adj, &instructions);

        assert_eq!(result, 5031);
    }
}
