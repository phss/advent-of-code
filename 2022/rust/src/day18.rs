use std::collections::HashSet;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day18.txt").unwrap();
    let cubes = parse(lines);
    surface_area(&cubes)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day18.txt").unwrap();
    let cubes = parse(lines);
    outside_surface_area(&cubes)
}

fn surface_area(cubes: &Vec<(usize, usize, usize)>) -> usize {
    let mut area = 0;
    let mut previous = HashSet::new();
    let diffs: Vec<(isize, isize, isize)> = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    for cube in cubes {
        area += 6;
        let (x, y, z) = *cube;

        for (x_diff, y_diff, z_diff) in &diffs {
            if previous.contains(&(
                x.wrapping_add_signed(*x_diff),
                y.wrapping_add_signed(*y_diff),
                z.wrapping_add_signed(*z_diff),
            )) {
                area -= 2;
            }
        }

        previous.insert(cube);
    }

    area
}

fn outside_surface_area(cubes: &Vec<(usize, usize, usize)>) -> usize {
    let cubes: HashSet<_> = cubes.iter().cloned().collect();
    let visited: HashSet<usize> = HashSet::new();

    let start = cubes.iter().min().unwrap();
    let orientation = (0, 0, -1);

    visited.len()
}

fn to_3d_direction(
    orientation: &(isize, isize, isize),
    direction: &(isize, isize),
) -> (isize, isize, isize) {
    let (x_ori, y_ori, z_ori) = orientation.clone();
    let (x_2d_dir, y_2d_dir) = direction.clone();

    (
        x_2d_dir * z_ori.abs() + x_2d_dir * y_ori.abs(),
        y_2d_dir * z_ori.abs() + y_2d_dir * x_ori.abs(),
        x_2d_dir * x_ori.abs() + y_2d_dir * y_ori.abs(),
    )
}

fn parse(input: Vec<String>) -> Vec<(usize, usize, usize)> {
    input
        .into_iter()
        .map(|line| {
            let parts: Vec<usize> = line.split(',').map(|part| part.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::ops::Neg;

    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ];
        let input = input.iter().map(|s| s.to_string()).collect();
        let cubes = parse(input);

        let result = surface_area(&cubes);

        assert_eq!(result, 64);
    }

    #[test]
    fn sample_input_part_2() {
        let input = vec![
            "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6",
            "1,2,5", "3,2,5", "2,1,5", "2,3,5",
        ];
        let input = input.iter().map(|s| s.to_string()).collect();
        let cubes = parse(input);

        let result = outside_surface_area(&cubes);

        assert_eq!(result, 58);
    }

    #[test]
    fn test_to_3d_direction() {
        let left = (-1, 0);
        let right = (1, 0);
        let up = (0, -1);
        let down = (0, 1);

        let top = (0, 0, -1);
        assert_eq!(to_3d_direction(&top, &left), (-1, 0, 0));
        assert_eq!(to_3d_direction(&top, &right), (1, 0, 0));
        assert_eq!(to_3d_direction(&top, &up), (0, -1, 0));
        assert_eq!(to_3d_direction(&top, &down), (0, 1, 0));

        let bottom = (0, 0, 1);
        assert_eq!(to_3d_direction(&bottom, &left), (-1, 0, 0));
        assert_eq!(to_3d_direction(&bottom, &right), (1, 0, 0));
        assert_eq!(to_3d_direction(&bottom, &up), (0, -1, 0));
        assert_eq!(to_3d_direction(&bottom, &down), (0, 1, 0));

        let east = (-1, 0, 0);
        assert_eq!(to_3d_direction(&east, &left), (0, 0, -1));
        assert_eq!(to_3d_direction(&east, &right), (0, 0, 1));
        assert_eq!(to_3d_direction(&east, &up), (0, -1, 0));
        assert_eq!(to_3d_direction(&east, &down), (0, 1, 0));

        let west = (1, 0, 0);
        assert_eq!(to_3d_direction(&west, &left), (0, 0, -1));
        assert_eq!(to_3d_direction(&west, &right), (0, 0, 1));
        assert_eq!(to_3d_direction(&west, &up), (0, -1, 0));
        assert_eq!(to_3d_direction(&west, &down), (0, 1, 0));

        let north = (0, -1, 0);
        assert_eq!(to_3d_direction(&north, &left), (-1, 0, 0));
        assert_eq!(to_3d_direction(&north, &right), (1, 0, 0));
        assert_eq!(to_3d_direction(&north, &up), (0, 0, -1));
        assert_eq!(to_3d_direction(&north, &down), (0, 0, 1));

        let south = (0, 1, 0);
        assert_eq!(to_3d_direction(&south, &left), (-1, 0, 0));
        assert_eq!(to_3d_direction(&south, &right), (1, 0, 0));
        assert_eq!(to_3d_direction(&south, &up), (0, 0, -1));
        assert_eq!(to_3d_direction(&south, &down), (0, 0, 1));
    }
}
