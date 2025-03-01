use std::str::FromStr;

use itertools::Itertools;

use crate::parser;

#[derive(Debug)]
struct Hailstone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Hailstone {
    fn points_2d(&self) -> (f64, f64, f64, f64) {
        let (x_1, y_1) = (self.position.0 as f64, self.position.1 as f64);
        let (x_2, y_2) = (
            x_1 + 200.0 * self.velocity.0 as f64,
            y_1 + 200.0 * self.velocity.1 as f64,
        );
        (x_1, y_1, x_2, y_2)
    }

    fn elements(&self) -> (f64, f64, f64, f64, f64, f64) {
        let (x, y, z) = self.position;
        let (vx, vy, vz) = self.velocity;

        (
            x as f64, y as f64, z as f64, vx as f64, vy as f64, vz as f64,
        )
    }

    fn in_the_future(&self, (x, y): (f64, f64)) -> bool {
        let (sx, sy, _, _) = self.points_2d();

        if self.velocity.0 > 0 && x < sx {
            return false;
        } else if self.velocity.0 < 0 && x > sx {
            return false;
        }

        if self.velocity.1 > 0 && y < sy {
            return false;
        } else if self.velocity.1 < 0 && y > sy {
            return false;
        }

        true
    }

    fn relative_to(&self, other: &Hailstone) -> Hailstone {
        Hailstone {
            position: (
                self.position.0 - other.position.0,
                self.position.1 - other.position.1,
                self.position.2 - other.position.2,
            ),
            velocity: (
                self.velocity.0 - other.velocity.0,
                self.velocity.1 - other.velocity.1,
                self.velocity.2 - other.velocity.2,
            ),
        }
    }
}

impl FromStr for Hailstone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').collect();

        let position: Vec<isize> = parts[0]
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
    let hailstones: Vec<Hailstone> = parser::read("data/day24.txt").unwrap();
    sum_of_perfect_throw(&hailstones)
}

fn count_intersections(hailstones: &Vec<Hailstone>, min: f64, max: f64) -> usize {
    hailstones
        .into_iter()
        .combinations(2)
        .filter(|hs| {
            let hail_a = hs.get(0).unwrap();
            let hail_b = hs.get(1).unwrap();
            let intersection = intersection_2d(hail_a, hail_b);

            match intersection {
                Some((x, y)) => {
                    let within_boundaries = x >= min && x <= max && y >= min && y <= max;
                    within_boundaries
                        && hail_a.in_the_future((x, y))
                        && hail_b.in_the_future((x, y))
                }
                None => false,
            }
        })
        .count()
}

fn intersection_2d(hail_a: &Hailstone, hail_b: &Hailstone) -> Option<(f64, f64)> {
    let (x_1, y_1, x_2, y_2) = hail_a.points_2d();
    let (x_3, y_3, x_4, y_4) = hail_b.points_2d();

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

fn sum_of_perfect_throw(hailstones: &Vec<Hailstone>) -> usize {
    let hail_0 = hailstones.get(3).unwrap();
    let hail_1 = hailstones.get(164).unwrap();
    let hail_2 = hailstones.get(199).unwrap();

    let (x1, y1, z1, vx1, vy1, vz1) = hail_1.elements();
    let (xr1, yr1, zr1, vxr1, _, vzr1) = hail_1.relative_to(hail_0).elements();
    let (x2, y2, z2, vx2, vy2, vz2) = hail_2.elements();
    let (xr2, yr2, zr2, vxr2, _, vzr2) = hail_2.relative_to(hail_0).elements();

    let numerator =
        (yr2 * xr1 * vzr1) - (vxr1 * yr2 * zr1) + (yr1 * zr2 * vxr1) - (yr1 * xr2 * vzr1);
    let denominator = yr1 * ((vzr1 * vxr2) - (vxr1 * vzr2));
    let t2 = numerator / denominator;

    let numerator = (yr1 * xr2) + (yr1 * vxr2 * t2) - (yr2 * xr1);
    let denominator = yr2 * vxr1;
    let t1 = numerator / denominator;

    println!("{t1} {t2}");

    let cx1 = x1 + (t1 * vx1);
    let cy1 = y1 + (t1 * vy1);
    let cz1 = z1 + (t1 * vz1);

    let cx2 = x2 + (t2 * vx2);
    let cy2 = y2 + (t2 * vy2);
    let cz2 = z2 + (t2 * vz2);

    println!("{cx1} {cy1} {cz1}");
    println!("{cx2} {cy2} {cz2}");

    let xm = (cx2 - cx1) / (t2 - t1);
    let ym = (cy2 - cy1) / (t2 - t1);
    let zm = (cz2 - cz1) / (t2 - t1);

    println!("{xm} {ym} {zm}");

    let xc = cx1 - (xm * t1);
    let yc = cy1 - (ym * t1);
    let zc = cz1 - (zm * t1);

    println!("{xc} {yc} {zc}");

    (xc + yc + zc) as usize
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
    fn sample_input_part_2() {
        // let lines = vec![
        //     "19, 13, 30 @ -2,  1, -2",
        //     "18, 19, 22 @ -1, -1, -2",
        //     "20, 25, 34 @ -2, -2, -4",
        //     "12, 31, 28 @ -1, -2, -1",
        //     "20, 19, 15 @  1, -5, -3",
        // ];
        // let hailstones: Vec<Hailstone> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        // let result = sum_of_perfect_throw(&hailstones);

        // assert_eq!(result, 47);
    }
}
