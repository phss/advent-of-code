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
    let reports: Vec<Report> = parser::read("data/day2.txt").unwrap();
    safe_reports_with_dampening(&reports)
}

fn safe_reports(reports: &Vec<Report>) -> u32 {
    reports
        .iter()
        .filter(|report| safe_levels(&report.levels))
        .count() as u32
}

fn safe_reports_with_dampening(reports: &Vec<Report>) -> u32 {
    let mut safe = 0;

    for report in reports {
        let mut candidates = vec![report.levels.clone()];
        for i in 0..report.levels.len() {
            let mut candidate = report.levels.clone();
            candidate.remove(i);
            candidates.push(candidate);
        }

        if candidates.iter().any(|report| safe_levels(report)) {
            safe += 1;
        }
    }

    safe
}

fn safe_levels(levels: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let at_most_3 = diffs.iter().all(|&diff| diff.abs() <= 3);
    let all_increasing = diffs.iter().all(|&diff| diff > 0);
    let all_decreasing = diffs.iter().all(|&diff| diff < 0);

    at_most_3 && (all_increasing || all_decreasing)
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
    fn sample_input_part_2() {
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

        let result = safe_reports_with_dampening(&reports);

        assert_eq!(result, 4);
    }
}
