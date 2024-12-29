use std::collections::HashSet;

use crate::parser;

type Map = Vec<Vec<u32>>;
type Coord = (usize, usize);

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day10.txt").unwrap();
    let map = parse(&lines);
    sum_trailheads(&map)
}

pub fn part2() -> u32 {
    0
}

fn sum_trailheads(map: &Map) -> u32 {
    find_trailheads(map)
        .iter()
        .map(|trailhead| count_trails(map, &trailhead))
        .sum()
}

fn parse(raw: &Vec<String>) -> Vec<Vec<u32>> {
    raw.iter()
        .map(|row| {
            row.chars()
                .map(|num| num.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn find_trailheads(map: &Map) -> Vec<Coord> {
    let mut trailheads = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == 0 {
                trailheads.push((x, y));
            }
        }
    }

    trailheads
}

fn count_trails(map: &Map, trailhead: &Coord) -> u32 {
    let map_width = map[0].len() - 1;
    let map_height = map.len() - 1;

    let mut trailends = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = vec![*trailhead];

    while let Some(coord) = to_visit.pop() {
        let (x, y) = coord;
        let height = map[y][x];

        visited.insert(coord);

        if height == 9 {
            trailends.insert(coord);
            continue;
        }

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for direction in directions.iter() {
            let (dir_x, dir_y) = *direction;

            let new_x = x.checked_add_signed(dir_x).unwrap_or(x).min(map_width);
            let new_y = y.checked_add_signed(dir_y).unwrap_or(y).min(map_height);
            let new_coord = (new_x, new_y);
            let new_coord_height = map[new_y][new_x];
            let is_single_step = new_coord_height.checked_sub(height) == Some(1);

            if is_single_step && !visited.contains(&new_coord) {
                to_visit.push(new_coord);
            }
        }
    }

    trailends.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let map = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];

        let result = sum_trailheads(&map);

        assert_eq!(result, 36)
    }

    #[test]
    fn sample_input_part_2() {}
}
