use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day23.txt").unwrap();
    let elves = parse(lines);
    count_empty_grounds(elves)
}

pub fn part2() -> usize {
    0
}

fn count_empty_grounds(elves: Vec<(isize, isize)>) -> usize {
    let final_elves = (0..10).fold(elves, |acc, _| round_move(acc));

    let xs: Vec<isize> = final_elves.iter().map(|elf| elf.0).collect();
    let ys: Vec<isize> = final_elves.iter().map(|elf| elf.0).collect();

    let min_x = xs.iter().min().unwrap();
    let max_x = xs.iter().max().unwrap();
    let min_y = ys.iter().min().unwrap();
    let max_y = ys.iter().max().unwrap();

    ((1 + max_x - min_x) * (1 + max_y - min_y)) as usize - final_elves.len()
}

fn round_move(elves: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    elves.clone()
}

fn parse(lines: Vec<String>) -> Vec<(isize, isize)> {
    let mut elves = Vec::new();

    for (y, row) in lines.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                elves.push((x as isize, y as isize));
            }
        }
    }

    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "..............",
            "..............",
            ".......#......",
            ".....###.#....",
            "...#...#.#....",
            "....#...##....",
            "...#.###......",
            "...##.#.##....",
            "....#..#......",
            "..............",
            "..............",
            "..............",
        ];
        let lines: Vec<String> = input.iter().map(|s| s.parse().unwrap()).collect();
        let elves = parse(lines);

        println!("{:?}", elves);

        let result = count_empty_grounds(elves);

        assert_eq!(result, 110);
    }

    #[test]
    fn sample_input_part_2() {}
}
