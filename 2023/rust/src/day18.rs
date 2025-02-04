use std::str::FromStr;

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
    let plan: Vec<Dig> = parser::read("data/day18.txt").unwrap();
    let fixed_plan = unscramble(plan);
    capacity(&fixed_plan)
}

fn capacity(plan: &Vec<Dig>) -> usize {
    let coords = to_coordinates(plan);
    area(&coords) + perimeter(&coords) / 2 + 1
}

fn area(coords: &Vec<(usize, usize)>) -> usize {
    let xs: Vec<usize> = coords.iter().map(|(x, _)| *x).collect();
    let ys: Vec<usize> = coords.iter().map(|(_, y)| *y).collect();

    let a: usize = xs[..xs.len() - 1]
        .iter()
        .zip(ys[1..].iter())
        .map(|(x, y)| x * y)
        .sum();

    let b: usize = xs[1..]
        .iter()
        .zip(ys[..ys.len() - 1].iter())
        .map(|(x, y)| x * y)
        .sum();

    (a - b) / 2
}

fn perimeter(coords: &Vec<(usize, usize)>) -> usize {
    coords
        .windows(2)
        .map(|cs| {
            let (x1, y1) = (cs[0].0 as isize, cs[0].1 as isize);
            let (x2, y2) = (cs[1].0 as isize, cs[1].1 as isize);

            (x1 - x2).abs() as usize + (y1 - y2).abs() as usize
        })
        .sum()
}

fn to_coordinates(plan: &Vec<Dig>) -> Vec<(usize, usize)> {
    let mut coords: Vec<(isize, isize)> = Vec::new();
    let mut current = (0, 0);
    coords.push(current);

    for dig in plan {
        for _ in 1..=dig.meters {
            current = match dig.direction {
                'R' => (current.0 + 1, current.1),
                'L' => (current.0 - 1, current.1),
                'D' => (current.0, current.1 + 1),
                'U' => (current.0, current.1 - 1),
                _ => panic!("unreacheable"),
            };
        }
        coords.push(current);
    }

    let min_x = coords.iter().map(|(x, _)| x).min().unwrap();
    let min_y = coords.iter().map(|(_, y)| y).min().unwrap();

    coords
        .iter()
        .map(|(x, y)| ((x + min_x.abs()) as usize, (y + min_y.abs()) as usize))
        .collect()
}

fn unscramble(plan: Vec<Dig>) -> Vec<Dig> {
    plan.iter()
        .map(|dig| {
            let direction = match dig.color.chars().nth(6).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("unreacheable"),
            };
            let meters = usize::from_str_radix(&dig.color[1..6], 16).unwrap();
            Dig {
                direction,
                meters,
                color: "blah".to_string(),
            }
        })
        .collect()
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
    fn sample_input_part_2() {
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
        let fixed_plan = unscramble(plan);

        let result = capacity(&fixed_plan);

        assert_eq!(result, 952408144115);
    }
}
