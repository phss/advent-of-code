use std::str::FromStr;

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
    0
}

pub fn part2() -> usize {
    0
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

        println!("{:?}", records);
    }

    #[test]
    fn sample_input_part_2() {}
}
