use std::str::FromStr;

use cached::proc_macro::cached;

use crate::parser;

#[derive(Debug)]
struct Record {
    field: String,
    damaged: Vec<usize>,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_string());
        }

        let field = parts[0].to_string();
        let damaged: Result<Vec<usize>, _> =
            parts[1].split(',').map(|x| x.parse::<usize>()).collect();

        match damaged {
            Ok(damaged) => Ok(Record { field, damaged }),
            Err(_) => Err("Failed to parse damaged indices".to_string()),
        }
    }
}

pub fn part1() -> usize {
    let records: Vec<Record> = parser::read("data/day12.txt").unwrap();
    sum_of_arrangements(&records)
}

pub fn part2() -> usize {
    let records: Vec<Record> = parser::read("data/day12.txt").unwrap();
    let unfolded_records = unfold(records);
    sum_of_arrangements(&unfolded_records)
}

fn sum_of_arrangements(records: &Vec<Record>) -> usize {
    records
        .iter()
        .map(|record| arrangements(record.field.clone(), record.damaged.clone()))
        .sum()
}

#[cached]
fn arrangements(field: String, damaged: Vec<usize>) -> usize {
    if field.len() == 0 {
        return if damaged.len() == 0 { 1 } else { 0 };
    }

    let mut count = 0;

    let head = field.chars().nth(0).unwrap();
    if head == '.' || head == '?' {
        count += arrangements(field[1..].to_string(), damaged.clone());
    }

    if head == '#' || head == '?' {
        if let Some(damage) = damaged.first() {
            let can_fit_damage =
                *damage <= field.len() && field[0..*damage].chars().all(|c| c == '#' || c == '?');
            let followed_by_space =
                *damage == field.len() || field.chars().nth(*damage) != Some('#');

            if can_fit_damage && followed_by_space {
                let damage_size = if *damage == field.len() {
                    *damage
                } else {
                    *damage + 1
                };
                count += arrangements(field[damage_size..].to_string(), damaged[1..].to_vec());
            }
        }
    }

    count
}

fn unfold(records: Vec<Record>) -> Vec<Record> {
    records
        .iter()
        .map(|record| {
            let mut field = record.field.clone();
            let mut damaged = record.damaged.clone();

            for _ in 0..4 {
                field.push('?');
                field.push_str(&record.field);

                damaged.extend(record.damaged.clone());
            }

            Record { field, damaged }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];
        let records: Vec<Record> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = sum_of_arrangements(&records);

        assert_eq!(result, 21);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];
        let records: Vec<Record> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let unfolded_records = unfold(records);

        let result = sum_of_arrangements(&unfolded_records);

        assert_eq!(result, 525152);
    }
}
