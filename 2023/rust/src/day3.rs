use crate::parser;

pub fn part1() -> u32 {
    let schematic: Vec<String> = parser::read("data/day3.txt").unwrap();
    sum_part_numbers(&schematic) as u32
}

pub fn part2() -> u32 {
    0
}

fn sum_part_numbers(schematic: &Vec<String>) -> usize {
    let directions: Vec<(isize, isize)> = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let width = schematic[0].len();
    let height = schematic.len();
    let mut sum = 0;

    for (y, line) in schematic.iter().enumerate() {
        let candidates = digits_from(line);

        for candidate in candidates {
            let is_part_number = candidate.iter().any(|(x, _)| {
                directions.iter().any(|(dir_x, dir_y)| {
                    let new_x = x.checked_add_signed(*dir_x).unwrap_or(0).min(width - 1);
                    let new_y = y.checked_add_signed(*dir_y).unwrap_or(0).min(height - 1);
                    let new_c = schematic[new_y].chars().nth(new_x).unwrap();

                    new_c != '.' && !new_c.is_ascii_digit()
                })
            });

            if is_part_number {
                let mut part_number = String::new();
                for (_, c) in candidate {
                    part_number.push(c);
                }
                sum += part_number.parse::<usize>().unwrap();
            }
        }
    }

    sum
}

fn digits_from(line: &String) -> Vec<Vec<(usize, char)>> {
    let mut digit_groups = Vec::new();
    let digits: Vec<(usize, char)> = line
        .char_indices()
        .filter(|(_, c)| c.is_ascii_digit())
        .collect();

    let mut current_group: Vec<(usize, char)> = Vec::new();
    for (i, c) in digits {
        if current_group.is_empty() || i == current_group.last().unwrap().0 + 1 {
            current_group.push((i, c));
        } else {
            digit_groups.push(current_group);
            current_group = vec![(i, c)];
        }
    }
    if !current_group.is_empty() {
        digit_groups.push(current_group);
    }

    digit_groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let schematic = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let schematic: Vec<String> = schematic.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = sum_part_numbers(&schematic);

        assert_eq!(result, 4361);
    }

    #[test]
    fn sample_input_part_2() {}
}
