use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    let maps = parse(&lines);
    note_summary(&maps, 0)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day13.txt").unwrap();
    let maps = parse(&lines);
    note_summary(&maps, 1)
}

fn note_summary(maps: &Vec<Vec<Vec<char>>>, smudge_limit: usize) -> usize {
    maps.iter()
        .map(|map| {
            mirror(map, smudge_limit)
                .map(|axis| axis * 100)
                .or(mirror(&transpose(map), smudge_limit))
                .unwrap_or(0)
        })
        .sum()
}

fn mirror(map: &Vec<Vec<char>>, smudge_limit: usize) -> Option<usize> {
    let height = map.len();

    for i in 0..height / 2 {
        let diffs: usize = (0..=i)
            .map(|j| {
                let a = i - j;
                let b = i + j + 1;
                map[a]
                    .iter()
                    .zip(map[b].iter())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum();

        if diffs == smudge_limit {
            return Some(i + 1);
        }

        let diffs: usize = (0..=i)
            .map(|j| {
                let a = height - 1 - i + j;
                let b = height - 1 - i - j - 1;
                map[a]
                    .iter()
                    .zip(map[b].iter())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum();

        if diffs == smudge_limit {
            return Some(height - 1 - i);
        }
    }

    None
}

fn transpose(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = map.len();
    let cols = map[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| map[row][col]).collect())
        .collect()
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Vec<char>>> {
    let parts = lines.split(String::is_empty);

    parts
        .map(|part| part.iter().map(|s| s.chars().collect()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let maps = parse(&lines);

        let result = note_summary(&maps, 0);

        assert_eq!(result, 405);
    }

    #[test]
    fn sample_input_part_1_additional() {
        let lines = vec![
            "##...##",
            "..#.#..",
            "#...#..",
            ".###.##",
            "##.#.##",
            "##..###",
            "##.....",
            ".#.##..",
            ".#..#..",
            "",
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
            "",
            ".#.##.#.#",
            ".##..##..",
            ".#.##.#..",
            "#......##",
            "#......##",
            ".#.##.#..",
            ".##..##.#",
            "",
            "#..#....#",
            "###..##..",
            ".##.#####",
            ".##.#####",
            "###..##..",
            "#..#....#",
            "#..##...#",
            "",
            "#.##..##.",
            "..#.##.#.",
            "##..#...#",
            "##...#..#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let maps = parse(&lines);

        let result = note_summary(&maps, 0);

        assert_eq!(result, 715);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let maps = parse(&lines);

        let result = note_summary(&maps, 1);

        assert_eq!(result, 400);
    }

    #[test]
    fn sample_input_part_2_additional() {
        let lines = vec![
            "##...##",
            "..#.#..",
            "#...#..",
            ".###.##",
            "##.#.##",
            "##..###",
            "##.....",
            ".#.##..",
            ".#..#..",
            "",
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
            "",
            ".#.##.#.#",
            ".##..##..",
            ".#.##.#..",
            "#......##",
            "#......##",
            ".#.##.#..",
            ".##..##.#",
            "",
            "#..#....#",
            "###..##..",
            ".##.#####",
            ".##.#####",
            "###..##..",
            "#..#....#",
            "#..##...#",
            "",
            "#.##..##.",
            "..#.##.#.",
            "##..#...#",
            "##...#..#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let maps = parse(&lines);

        let result = note_summary(&maps, 1);

        assert_eq!(result, 2200);
    }
}
