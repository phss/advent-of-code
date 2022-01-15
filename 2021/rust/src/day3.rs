use crate::parser;

fn to_number(binary_string: &String) -> u32 {
    let number = isize::from_str_radix(binary_string, 2).unwrap();
    number.try_into().unwrap()
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day3.txt").unwrap();
    let diagnostic_report = lines.iter().map(to_number).collect();
    power_consumption(diagnostic_report)
}

fn power_consumption(diagnostic_report: Vec<u32>) -> u32 {
    let half_reports: u32 = diagnostic_report.len().try_into().unwrap();
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    let length_of_bits = diagnostic_report
        .iter()
        .map(|number| 32 - number.leading_zeros())
        .max()
        .unwrap();

    for i in 0..length_of_bits {
        let mut ones = 0;
        for number in diagnostic_report.iter() {
            ones += number >> i & 1;
        }
        let most_common = (ones > (half_reports / 2)) as u32;
        let least_common = (ones < (half_reports / 2)) as u32;

        gamma_rate += most_common << i;
        epsilon_rate += least_common << i;
    }

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_power_consumption() {
        let diagnostic_report: Vec<u32> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(power_consumption(diagnostic_report), 198);
    }
}
