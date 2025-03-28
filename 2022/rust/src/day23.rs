use crate::parser;
use itertools::Itertools;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day23.txt").unwrap();
    let elves = parse(lines);
    count_empty_grounds(elves)
}

pub fn part2() -> usize {
    0
}

fn count_empty_grounds(elves: Vec<(isize, isize)>) -> usize {
    let mut directions = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let final_elves = (0..10).fold(elves, |acc, _| {
        let new_elves = round_move(acc, &directions);
        directions.rotate_left(1);
        print(&new_elves);
        println!();
        new_elves
    });

    let xs: Vec<isize> = final_elves.iter().map(|elf| elf.0).collect();
    let ys: Vec<isize> = final_elves.iter().map(|elf| elf.0).collect();

    let min_x = xs.iter().min().unwrap();
    let max_x = xs.iter().max().unwrap();
    let min_y = ys.iter().min().unwrap();
    let max_y = ys.iter().max().unwrap();

    ((1 + max_x - min_x) * (1 + max_y - min_y)) as usize - final_elves.len()
}

fn round_move(elves: Vec<(isize, isize)>, directions: &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let new_elves: Vec<(isize, isize)> = elves
        .iter()
        .map(|(elf_x, elf_y)| {
            let (dir_x, dir_y) = directions
                .iter()
                .find(|(dir_x, dir_y)| {
                    if *dir_x != 0 {
                        !elves.contains(&(*elf_x + dir_x, *elf_y))
                            && !elves.contains(&(*elf_x + dir_x, *elf_y + 1))
                            && !elves.contains(&(*elf_x + dir_x, *elf_y - 1))
                    } else {
                        !elves.contains(&(*elf_x, *elf_y + dir_y))
                            && !elves.contains(&(*elf_x + 1, *elf_y + dir_y))
                            && !elves.contains(&(*elf_x - 1, *elf_y + dir_y))
                    }
                })
                .unwrap_or(&(0, 0));

            (elf_x + dir_x, elf_y + dir_y)
        })
        .collect();

    let position_count = new_elves.iter().counts();
    new_elves
        .iter()
        .enumerate()
        .map(|(i, elf)| {
            let elves_count = *position_count.get(&elf).unwrap();
            if elves_count > 1 {
                elves[i]
            } else {
                *elf
            }
        })
        .collect()
}

fn print(elves: &Vec<(isize, isize)>) {
    let xs: Vec<isize> = elves.iter().map(|elf| elf.0).collect();
    let ys: Vec<isize> = elves.iter().map(|elf| elf.0).collect();

    let min_x = xs.iter().min().unwrap();
    let max_x = xs.iter().max().unwrap();
    let min_y = ys.iter().min().unwrap();
    let max_y = ys.iter().max().unwrap();

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    let adjusted_elves: Vec<(usize, usize)> = elves
        .iter()
        .map(|elf| ((elf.0 - min_x) as usize, (elf.1 - min_y) as usize))
        .collect();

    for y in 0..height {
        for x in 0..width {
            if adjusted_elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
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

        print(&elves);

        let result = count_empty_grounds(elves);

        assert_eq!(result, 110);
    }

    #[test]
    fn sample_input_part_2() {}
}
