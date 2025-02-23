use std::collections::HashSet;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day23.txt").unwrap();
    let map = parse(&lines);
    longest_hike(&map, HashSet::new(), (1, 0), (139, 140))
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day23.txt").unwrap();
    let map = parse(&lines);
    longest_hike_no_slopes(&map, HashSet::new(), (1, 0), (139, 140))
}

fn longest_hike(
    map: &Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    from: (usize, usize),
    to: (usize, usize),
) -> usize {
    if from == to {
        return visited.len();
    }

    let mut new_visited = visited.clone();
    new_visited.insert(from);

    let (x, y) = from;
    let mut max_steps = 0;

    let directions = [(1, 0, '>'), (-1, 0, '<'), (0, 1, 'v'), (0, -1, '^')];
    for (dir_x, dir_y, dir_char) in directions {
        let next_x = x.checked_add_signed(dir_x).unwrap_or(0);
        let next_y = y.checked_add_signed(dir_y).unwrap_or(0);
        let next_position = (next_x, next_y);
        let next_char = map[next_y][next_x];

        if new_visited.contains(&next_position) || (next_char != '.' && next_char != dir_char) {
            continue;
        }

        max_steps = max_steps.max(longest_hike(map, new_visited.clone(), next_position, to));
    }

    max_steps
}

fn longest_hike_no_slopes(
    map: &Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    from: (usize, usize),
    to: (usize, usize),
) -> usize {
    if from == to {
        return visited.len();
    }

    let mut new_visited = visited.clone();
    new_visited.insert(from);

    let (x, y) = from;
    let mut max_steps = 0;

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for (dir_x, dir_y) in directions {
        let next_x = x.checked_add_signed(dir_x).unwrap_or(0);
        let next_y = y.checked_add_signed(dir_y).unwrap_or(0);
        let next_position = (next_x, next_y);
        let next_char = map[next_y][next_x];

        if new_visited.contains(&next_position) || next_char == '#' {
            continue;
        }

        max_steps = max_steps.max(longest_hike_no_slopes(
            map,
            new_visited.clone(),
            next_position,
            to,
        ));
    }

    max_steps
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
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = longest_hike(&map, HashSet::new(), (1, 0), (21, 22));

        assert_eq!(result, 94);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = longest_hike_no_slopes(&map, HashSet::new(), (1, 0), (21, 22));

        assert_eq!(result, 154);
    }
}
