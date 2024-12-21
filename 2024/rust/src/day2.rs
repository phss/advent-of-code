use std::str::FromStr;

use crate::parser;

struct Report {
    levels: Vec<i32>,
}

#[derive(Debug, Clone)]
struct ParseReportError;

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels: Vec<i32> = s
            .split_whitespace()
            .map(|level| level.parse().unwrap())
            .collect();
        Ok(Report { levels })
    }
}

pub fn part1() -> u32 {
    let reports: Vec<Report> = parser::read("data/day2.txt").unwrap();
    safe_reports(&reports)
}

pub fn part2() -> u32 {
    0
}

fn safe_reports(reports: &Vec<Report>) -> u32 {
    let mut safe = 0;

    for report in reports {
        let diffs: Vec<i32> = report
            .levels
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();

        let at_most_3 = diffs.iter().all(|&diff| diff.abs() <= 3);
        let all_increasing = diffs.iter().all(|&diff| diff > 0);
        let all_decreasing = diffs.iter().all(|&diff| diff < 0);

        if at_most_3 && (all_increasing || all_decreasing) {
            safe += 1;
        }
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let reports = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];

        let result = safe_reports(&reports);

        assert_eq!(result, 2);
    }

    #[test]
    fn sample_input_part_2() {}
}
