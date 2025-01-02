use std::str::FromStr;

use crate::parser;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (usize, usize),
    velocities: (isize, isize),
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let position_part = parts[0]
            .trim_start_matches("p=")
            .split(',')
            .collect::<Vec<&str>>();
        let velocities_part = parts[1]
            .trim_start_matches("v=")
            .split(',')
            .collect::<Vec<&str>>();

        let position = (
            position_part[0].parse().unwrap(),
            position_part[1].parse().unwrap(),
        );

        let velocities = (
            velocities_part[0].parse().unwrap(),
            velocities_part[1].parse().unwrap(),
        );

        Ok(Robot {
            position,
            velocities,
        })
    }
}

pub fn part1() -> u32 {
    let robots: Vec<Robot> = parser::read("data/day14.txt").unwrap();
    safety_factor_after(&robots, 101, 103, 100)
}

pub fn part2() -> u32 {
    0
}

fn safety_factor_after(robots: &Vec<Robot>, width: usize, height: usize, seconds: u32) -> u32 {
    let width = width as isize;
    let height = height as isize;
    let seconds = seconds as isize;
    let calc = |value: usize, velocity: isize, dimension: isize| -> usize {
        let mut new_value = value as isize + (velocity * seconds);
        let wrapped_value = new_value.abs() % dimension;
        if new_value >= 0 || wrapped_value == 0 {
            new_value %= dimension;
        } else {
            new_value = dimension - wrapped_value;
        }
        new_value as usize
    };

    let robots = robots
        .iter()
        .map(|robot| {
            let (x, y) = robot.position;
            let (vel_x, vel_y) = robot.velocities;

            Robot {
                position: (calc(x, vel_x, width), calc(y, vel_y, height)),
                velocities: robot.velocities,
            }
        })
        .collect();

    safety_factor(&robots, width as usize, height as usize)
}

fn safety_factor(robots: &Vec<Robot>, width: usize, height: usize) -> u32 {
    let middle_x = width / 2;
    let middle_y = height / 2;
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        let in_first_half_x = (0..middle_x).contains(&robot.position.0);
        let in_second_half_x = ((middle_x + 1)..width).contains(&robot.position.0);
        let in_first_half_y = (0..middle_y).contains(&robot.position.1);
        let in_second_half_y = ((middle_y + 1)..height).contains(&robot.position.1);
        match (
            in_first_half_x,
            in_second_half_x,
            in_first_half_y,
            in_second_half_y,
        ) {
            (true, false, true, false) => q1 += 1,
            (false, true, true, false) => q2 += 1,
            (true, false, false, true) => q3 += 1,
            (false, true, false, true) => q4 += 1,
            _ => { /* in the middle lines */ }
        };
    }

    let mut factor = 1;
    if q1 > 0 {
        factor *= q1;
    };
    if q2 > 0 {
        factor *= q2;
    };
    if q3 > 0 {
        factor *= q3;
    };
    if q4 > 0 {
        factor *= q4;
    };

    factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ];
        let robots = lines.iter().map(|s| s.parse().unwrap()).collect();

        let result = safety_factor_after(&robots, 11, 7, 100);

        assert_eq!(result, 12)
    }

    #[test]
    fn sample_input_part_2() {}
}
