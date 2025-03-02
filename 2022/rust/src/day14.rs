use std::collections::HashSet;

use crate::parser;

type Point = (usize, usize);
type Path = Vec<Point>;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day14.txt").unwrap();
    let paths: Vec<Path> = lines.iter().map(|line| parse(line)).collect();
    resting_sands(&paths)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day14.txt").unwrap();
    let paths: Vec<Path> = lines.iter().map(|line| parse(line)).collect();
    resting_sands_with_floor(&paths)
}

fn parse(raw: &str) -> Path {
    raw.split(" -> ")
        .map(|point_raw| {
            let mut elements = point_raw.split(",");
            (
                elements.next().unwrap().parse().unwrap(),
                elements.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn resting_sands(paths: &Vec<Path>) -> usize {
    let mut sands = 0;
    let mut occupied: HashSet<Point> = rock_places(paths);
    let max_y = *occupied.iter().map(|(_, y)| y).max().unwrap();

    loop {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y > max_y {
                return sands;
            }

            let occupied_down = occupied.contains(&(x, y + 1));
            let occupied_down_left = occupied.contains(&(x - 1, y + 1));
            let occupied_down_right = occupied.contains(&(x + 1, y + 1));

            if occupied_down && occupied_down_left && occupied_down_right {
                occupied.insert((x, y));
                sands += 1;
                break;
            }

            if occupied_down && !occupied_down_left {
                x -= 1;
            } else if occupied_down && !occupied_down_right {
                x += 1;
            }
            y += 1;
        }
    }
}

fn resting_sands_with_floor(paths: &Vec<Path>) -> usize {
    let mut sands = 0;
    let mut occupied: HashSet<Point> = rock_places(paths);
    let max_y = *occupied.iter().map(|(_, y)| y).max().unwrap();
    let floor_y = max_y + 2;

    loop {
        let mut x = 500;
        let mut y = 0;
        loop {
            let floor_is_down = (y + 1) == floor_y;
            let occupied_down = floor_is_down || occupied.contains(&(x, y + 1));
            let occupied_down_left = floor_is_down || occupied.contains(&(x - 1, y + 1));
            let occupied_down_right = floor_is_down || occupied.contains(&(x + 1, y + 1));

            if occupied_down && occupied_down_left && occupied_down_right {
                occupied.insert((x, y));
                sands += 1;

                if y == 0 {
                    return sands;
                }
                break;
            }

            if occupied_down && !occupied_down_left {
                x -= 1;
            } else if occupied_down && !occupied_down_right {
                x += 1;
            }
            y += 1;
        }
    }
}

fn rock_places(paths: &Vec<Path>) -> HashSet<Point> {
    let mut rocks: HashSet<Point> = HashSet::new();

    for path in paths {
        path.windows(2).for_each(|segment| {
            let (fx, fy) = segment[0];
            let (tx, ty) = segment[1];

            if fx != tx {
                (fx.min(tx)..=fx.max(tx)).for_each(|x| {
                    rocks.insert((x, fy));
                });
            } else if fy != ty {
                (fy.min(ty)..=fy.max(ty)).for_each(|y| {
                    rocks.insert((fx, y));
                });
            }
        })
    }

    rocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            parse("503,4 -> 502,4 -> 502,9 -> 494,9"),
            vec![(503, 4), (502, 4), (502, 9), (494, 9)]
        );
    }

    #[test]
    fn rock_placements() {
        let paths = vec![
            parse("498,4 -> 498,6 -> 496,6"),
            parse("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];
        let expected_rock: HashSet<Point> = vec![
            (498, 4),
            (498, 5),
            (498, 6),
            (497, 6),
            (496, 6),
            (503, 4),
            (502, 4),
            (502, 5),
            (502, 6),
            (502, 7),
            (502, 8),
            (502, 9),
            (501, 9),
            (500, 9),
            (499, 9),
            (498, 9),
            (497, 9),
            (496, 9),
            (495, 9),
            (494, 9),
        ]
        .into_iter()
        .collect();
        assert_eq!(rock_places(&paths), expected_rock);
    }

    // #[test]
    // fn map() {
    //     let paths = vec![
    //         parse("498,4 -> 498,6 -> 496,6"),
    //         parse("503,4 -> 502,4 -> 502,9 -> 494,9"),
    //     ];
    // }

    #[test]
    fn sample_input_part_1() {
        let paths = vec![
            parse("498,4 -> 498,6 -> 496,6"),
            parse("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];
        assert_eq!(resting_sands(&paths), 24);
    }

    #[test]
    fn sample_input_part_2() {
        let paths = vec![
            parse("498,4 -> 498,6 -> 496,6"),
            parse("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];
        assert_eq!(resting_sands_with_floor(&paths), 93);
    }
}
