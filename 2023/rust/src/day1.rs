use core::num;

use crate::parser;

pub fn part1() -> u32 {
    let doc: Vec<String> = parser::read("data/day1.txt").unwrap();
    calibration_values(&doc) as u32
}

pub fn part2() -> u32 {
    0
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
    fn sample_input_part_2() {}
}
