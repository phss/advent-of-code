use std::str::FromStr;

use crate::parser;

#[derive(Debug)]
struct Hailstone {
    position: (usize, usize, usize),
    velocity: (isize, isize, isize),
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').collect();

        let position: Vec<usize> = parts[0]
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let velocity: Vec<isize> = parts[1]
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        Ok(Hailstone {
            position: (position[0], position[1], position[2]),
            velocity: (velocity[0], velocity[1], velocity[2]),
        })
    }
}

pub fn part1() -> usize {
    let hailstones: Vec<Hailstone> = parser::read("data/day24.txt").unwrap();
    count_intersections(&hailstones)
}

pub fn part2() -> usize {
    0
}

fn count_intersections(hailstones: &Vec<Hailstone>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "19, 13, 30 @ -2,  1, -2",
            "18, 19, 22 @ -1, -1, -2",
            "20, 25, 34 @ -2, -2, -4",
            "12, 31, 28 @ -1, -2, -1",
            "20, 19, 15 @  1, -5, -3",
        ];
        let hailstones: Vec<Hailstone> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_intersections(&hailstones);

        assert_eq!(result, 2);
    }

    #[test]
    fn sample_input_part_2() {}
}
