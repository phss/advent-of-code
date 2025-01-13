use std::usize;

use crate::parser;

pub fn part1() -> u32 {
    let doc: Vec<String> = parser::read("data/day1.txt").unwrap();
    calibration_values(&doc) as u32
}

pub fn part2() -> u32 {
    let doc: Vec<String> = parser::read("data/day1.txt").unwrap();
    calibration_values_with_str_number(&doc) as u32
}

fn calibration_values(doc: &Vec<String>) -> usize {
    doc.iter()
        .map(|line| {
            let number_chars: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let mut number_str = String::new();
            number_str.push(*number_chars.first().unwrap());
            number_str.push(*number_chars.last().unwrap());
            number_str.parse::<usize>().unwrap()
        })
        .sum()
}

fn calibration_values_with_str_number(doc: &Vec<String>) -> usize {
    let first_digits: Vec<String> = doc
        .iter()
        .map(|line| find_digit(line, line.char_indices()))
        .collect();
    let last_digits: Vec<String> = doc
        .iter()
        .map(|line| find_digit(line, line.char_indices().rev()))
        .collect();

    first_digits
        .iter()
        .zip(last_digits.iter())
        .map(|(a, b)| (a.clone() + b).parse::<usize>().unwrap())
        .sum()
}

fn find_digit<I>(line: &String, chars: I) -> String
where
    I: Iterator<Item = (usize, char)>,
{
    let number_conversion = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for (i, c) in chars {
        if c.is_ascii_digit() {
            return c.to_string();
        }
        let sub_line = &line[i..];
        for (s, n) in &number_conversion {
            if sub_line.starts_with(s) {
                return n.to_string();
            }
        }
    }

    panic!("digit not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let doc = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let doc: Vec<String> = doc.into_iter().map(|s| s.to_string()).collect();

        let result = calibration_values(&doc);

        assert_eq!(result, 142)
    }

    #[test]
    fn sample_input_part_2() {
        let doc = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let doc: Vec<String> = doc.into_iter().map(|s| s.to_string()).collect();

        let result = calibration_values_with_str_number(&doc);

        assert_eq!(result, 281)
    }
}
