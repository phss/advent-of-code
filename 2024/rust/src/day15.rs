use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    let (mut map, moves, initial_position) = parse(&lines);
    sum_of_gps_coords(&mut map, &moves, initial_position)
}

pub fn part2() -> u32 {
    // let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    0
}

fn sum_of_gps_coords(
    map: &mut Vec<Vec<char>>,
    moves: &Vec<char>,
    initial_position: (usize, usize),
) -> u32 {
    let (mut x, mut y) = initial_position;

    moves.iter().map(to_dir).for_each(|dir @ (dir_x, dir_y)| {
        let new_x = x.wrapping_add_signed(dir_x);
        let new_y = y.wrapping_add_signed(dir_y);

        let moved = match map[new_y][new_x] {
            '.' => true,
            'O' => push(map, (new_x, new_y), dir),
            _ => false,
        };

        if moved {
            map[y][x] = '.';
            map[new_y][new_x] = '@';
            x = new_x;
            y = new_y;
        }
    });

    print(map);
    score(map)
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
    initial_position: (usize, usize),
    (dir_x, dir_y): (isize, isize),
) -> bool {
    let (mut free_x, mut free_y) = initial_position;
    loop {
        free_x = free_x.wrapping_add_signed(dir_x);
        free_y = free_y.wrapping_add_signed(dir_y);

        match map[free_y][free_x] {
            '#' => return false,
            '.' => break,
            _ => {}
        }
    }

    let (mut x, mut y) = initial_position;
    map[y][x] = '.';
    while x != free_x || y != free_y {
        x = x.wrapping_add_signed(dir_x);
        y = y.wrapping_add_signed(dir_y);
        map[y][x] = 'O';
    }

    true
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

fn print(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

fn score(map: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' {
                total += x + (100 * y);
            }
        }
    }

    total as u32
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
        let (mut map, moves, initial_position) = parse(&lines);

        let result = sum_of_gps_coords(&mut map, &moves, initial_position);

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
        let (mut map, moves, initial_position) = parse(&lines);

        let result = sum_of_gps_coords(&mut map, &moves, initial_position);

        assert_eq!(result, 2028);
    }

    #[test]
    fn sample_input_part_2() {}
}
