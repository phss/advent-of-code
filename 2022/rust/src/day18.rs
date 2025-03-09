use std::collections::HashSet;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day18.txt").unwrap();
    let cubes = parse(lines);
    surface_area(&cubes)
}

pub fn part2() -> usize {
    0
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
    fn sample_input_part_2() {}
}
