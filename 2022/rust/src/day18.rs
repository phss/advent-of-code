use std::collections::{HashSet, VecDeque};

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
    let mut cubes: HashSet<_> = cubes.iter().cloned().collect();

    let max_coord = cubes
        .iter()
        .flat_map(|(x, y, z)| vec![x, y, z])
        .max()
        .unwrap()
        + 1;

    let mut steamed: HashSet<(usize, usize, usize)> = HashSet::new();
    steamed.insert((0, 0, 0));
    let mut path: VecDeque<(usize, usize, usize)> = VecDeque::new();
    path.push_back((0, 0, 0));

    let mut area = 0;

    let directions = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    while let Some((x, y, z)) = path.pop_front() {
        for (x_dir, y_dir, z_dir) in directions {
            let x_new = x.checked_add_signed(x_dir).unwrap_or(0).min(max_coord);
            let y_new = y.checked_add_signed(y_dir).unwrap_or(0).min(max_coord);
            let z_new = z.checked_add_signed(z_dir).unwrap_or(0).min(max_coord);
            let new_node = (x_new, y_new, z_new);

            if steamed.contains(&new_node) {
                continue;
            }

            if cubes.contains(&new_node) {
                // area += 1;
                continue;
            }

            steamed.insert(new_node);
            path.push_back(new_node);
        }
    }

    for (x, y, z) in &cubes {
        for (x_dir, y_dir, z_dir) in directions {
            let x_new = x.checked_add_signed(x_dir).unwrap_or(0).min(max_coord);
            let y_new = y.checked_add_signed(y_dir).unwrap_or(0).min(max_coord);
            let z_new = z.checked_add_signed(z_dir).unwrap_or(0).min(max_coord);
            let new_node = (x_new, y_new, z_new);

            if steamed.contains(&new_node) {
                area += 1;
            }
        }
    }

    area
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
}
