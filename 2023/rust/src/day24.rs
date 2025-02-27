use std::str::FromStr;

use itertools::Itertools;

use crate::parser;

#[derive(Debug)]
struct Hailstone {
    position: (usize, usize, usize),
    velocity: (isize, isize, isize),
}

impl Hailstone {
    fn points(&self) -> (f64, f64, f64, f64) {
        let (x_1, y_1) = (self.position.0 as f64, self.position.1 as f64);
        let (x_2, y_2) = (x_1 + self.velocity.0 as f64, y_1 + self.velocity.1 as f64);
        (x_1, y_1, x_2, y_2)
    }

    fn in_the_future(&self, (x, y): (f64, f64)) -> bool {
        let (sx, sy, _, _) = self.points();

        if self.velocity.0 > 0 && x < sx {
            // println!("1 {} {} {}", self.velocity.0, x, sx);
            return false;
        } else if self.velocity.0 < 0 && x > sx {
            // println!("2 {} {} {}", self.velocity.0, x, sx);
            return false;
        }

        if self.velocity.1 > 0 && y < sy {
            // println!("3 {} {} {}", self.velocity.1, y, sy);
            return false;
        } else if self.velocity.1 < 0 && y > sy {
            // println!("4 {} {} {}", self.velocity.1, y, sy);
            return false;
        }

        true
    }
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
    count_intersections(&hailstones, 200000000000000.0, 400000000000000.0)
}

pub fn part2() -> usize {
    0
}

fn count_intersections(hailstones: &Vec<Hailstone>, min: f64, max: f64) -> usize {
    hailstones
        .into_iter()
        .combinations(2)
        .filter(|hs| {
            let hail_a = hs.get(0).unwrap();
            let hail_b = hs.get(1).unwrap();
            let intersection = intersection_2d(hail_a, hail_b);

            let result = match intersection {
                Some((x, y)) => {
                    let within_boundaries = x >= min && x <= max && y >= min && y <= max;
                    within_boundaries
                        && hail_a.in_the_future((x, y))
                        && hail_b.in_the_future((x, y))
                }
                None => false,
            };

            // println!("Hailstone A: {:?}", hail_a);
            // println!("Hailstone B: {:?}", hail_b);
            // println!("Intersection: {:?}", intersection);
            // println!("Result: {:?}", result);
            // println!();

            result
        })
        .count()
}

fn intersection_2d(hail_a: &Hailstone, hail_b: &Hailstone) -> Option<(f64, f64)> {
    let (x_1, y_1, x_2, y_2) = hail_a.points();
    let (x_3, y_3, x_4, y_4) = hail_b.points();

    let denominator = (x_1 - x_2) * (y_3 - y_4) - (y_1 - y_2) * (x_3 - x_4);
    if denominator == 0.0 {
        return None;
    }

    let x_numerator = (x_1 * y_2 - y_1 * x_2) * (x_3 - x_4) - (x_1 - x_2) * (x_3 * y_4 - y_3 * x_4);
    let y_numerator = (x_1 * y_2 - y_1 * x_2) * (y_3 - y_4) - (y_1 - y_2) * (x_3 * y_4 - y_3 * x_4);

    let x = x_numerator / denominator;
    let y = y_numerator / denominator;

    Some((x, y))
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

        let result = count_intersections(&hailstones, 7.0, 27.0);

        assert_eq!(result, 2);
    }

    #[test]
    fn sample_input_part_2() {}
}
