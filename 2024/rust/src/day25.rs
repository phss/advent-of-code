use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day25.txt").unwrap();
    let (locks, keys) = parse(&lines);
    count_fits(&locks, &keys) as u32
}

pub fn part2() -> u32 {
    0
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let parts = lines.split(|line| line.is_empty());

    for part in parts {
        let mut heights = Vec::new();
        for col in 0..5 {
            let mut height = 0;
            for row in part {
                if row.chars().nth(col) == Some('#') {
                    height += 1;
                }
            }
            heights.push(height - 1);
        }

        if part[0] == "#####" {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

fn count_fits(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;

    for lock in locks {
        for key in keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| (l + k) <= 5) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
            ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
            "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
            "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();
        let (locks, keys) = parse(&lines);

        let result = count_fits(&locks, &keys);

        assert_eq!(result, 3)
    }

    #[test]
    fn sample_input_part_2() {}
}
