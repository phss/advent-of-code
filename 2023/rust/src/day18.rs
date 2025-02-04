use std::{collections::HashSet, str::FromStr};

use crate::parser;

#[derive(Debug)]
struct Dig {
    direction: char,
    meters: usize,
    color: String,
}

impl FromStr for Dig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid input".to_string());
        }

        let direction = parts[0].chars().next().ok_or("Invalid direction")?;
        let meters = parts[1].parse::<usize>().map_err(|_| "Invalid meters")?;
        let color = parts[2][1..8].to_string();

        Ok(Dig {
            direction,
            meters,
            color,
        })
    }
}

pub fn part1() -> usize {
    let plan: Vec<Dig> = parser::read("data/day18.txt").unwrap();
    capacity(&plan)
}

pub fn part2() -> usize {
    0
}

fn capacity(plan: &Vec<Dig>) -> usize {
    let borders = to_borders(plan);
    let width = borders.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = borders.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut inside = 0;
    for y in 0..height {
        let mut walls = 0;
        for x in 0..width {
            if borders.contains(&(x, y)) {
                print!("#");
                if x == 0 || !borders.contains(&(x - 1, y)) {
                    walls += 1;
                }
            } else if walls % 2 != 0 {
                print!("O");
                inside += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }

    borders.len() + inside
}

fn to_borders(plan: &Vec<Dig>) -> HashSet<(usize, usize)> {
    let mut borders: HashSet<(isize, isize)> = HashSet::new();
    let mut current = (0, 0);

    for dig in plan {
        for _ in 1..=dig.meters {
            current = match dig.direction {
                'R' => (current.0 + 1, current.1),
                'L' => (current.0 - 1, current.1),
                'D' => (current.0, current.1 + 1),
                'U' => (current.0, current.1 - 1),
                _ => panic!("unreacheable"),
            };
            borders.insert(current);
        }
    }

    let min_x = borders.iter().map(|(x, _)| x).min().unwrap();
    let min_y = borders.iter().map(|(_, y)| y).min().unwrap();

    borders
        .iter()
        .map(|(x, y)| ((x + min_x.abs()) as usize, (y + min_y.abs()) as usize))
        .collect()
}

fn print(borders: &HashSet<(usize, usize)>) {
    let width = borders.iter().map(|(x, _)| x).max().unwrap() + 1;
    let height = borders.iter().map(|(_, y)| y).max().unwrap() + 1;

    for y in 0..height {
        for x in 0..width {
            if borders.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ];
        let plan: Vec<Dig> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = capacity(&plan);

        assert_eq!(result, 62);
    }

    #[test]
    fn sample_input_part_2() {}
}
