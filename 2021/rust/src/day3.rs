use crate::parser;

mod binary {
    pub fn to_number(binary_string: &String) -> u32 {
        let number = isize::from_str_radix(binary_string, 2).unwrap();
        number.try_into().unwrap()
    }

    pub fn max_bits_length(numbers: &Vec<u32>) -> u32 {
        numbers
            .iter()
            .map(|number| 32 - number.leading_zeros())
            .max()
            .unwrap()
    }

    pub fn most_common_bit_at(position: u32, numbers: &Vec<u32>) -> u32 {
        comparison_bit_at(position, PartialOrd::ge, numbers)
    }

    pub fn least_common_bit_at(position: u32, numbers: &Vec<u32>) -> u32 {
        comparison_bit_at(position, PartialOrd::lt, numbers)
    }

    fn comparison_bit_at(
        position: u32,
        comparison: fn(&u32, &u32) -> bool,
        numbers: &Vec<u32>,
    ) -> u32 {
        let mut half_numbers: u32 = (numbers.len() / 2).try_into().unwrap();
        if numbers.len() % 2 == 1 {
            half_numbers += 1;
        }
        let mut ones = 0;

        for number in numbers.iter() {
            ones += number >> position & 1;
        }
        comparison(&ones, &half_numbers) as u32
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn max_length() {
            let numbers = vec![0b1, 0b11, 0b111111, 0b11];
            assert_eq!(max_bits_length(&numbers), 6)
        }

        #[test]
        fn most_common_bit() {
            let numbers = vec![0b00, 0b11, 0b01, 0b11];
            assert_eq!(most_common_bit_at(0, &numbers), 1)
        }

        #[test]
        fn most_common_bit_when_equally_common() {
            let numbers = vec![0b00, 0b11, 0b01, 0b11];
            assert_eq!(most_common_bit_at(1, &numbers), 1)
        }

        #[test]
        fn most_common_with_odd_sized_list() {
            let numbers = vec![0b00, 0b10, 0b01, 0b11, 0b00];
            assert_eq!(most_common_bit_at(0, &numbers), 0)
        }

        #[test]
        fn least_common_bit() {
            let numbers = vec![0b00, 0b11, 0b01, 0b11];
            assert_eq!(least_common_bit_at(0, &numbers), 0)
        }

        #[test]
        fn least_common_bit_with_odd_sized_list() {
            let numbers = vec![0b00, 0b10, 0b01, 0b11, 0b00];
            assert_eq!(least_common_bit_at(0, &numbers), 1)
        }

        #[test]
        fn least_common_bit_when_equally_common() {
            let numbers = vec![0b00, 0b11, 0b01, 0b11];
            assert_eq!(least_common_bit_at(1, &numbers), 0)
        }
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day3.txt").unwrap();
    let diagnostic_report = lines.iter().map(binary::to_number).collect();
    power_consumption(diagnostic_report)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day3.txt").unwrap();
    let diagnostic_report = lines.iter().map(binary::to_number).collect();
    life_support_rating(diagnostic_report)
}

fn power_consumption(diagnostic_report: Vec<u32>) -> u32 {
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for position in 0..binary::max_bits_length(&diagnostic_report) {
        let most_common = binary::most_common_bit_at(position, &diagnostic_report);
        let least_common = binary::least_common_bit_at(position, &diagnostic_report);

        gamma_rate += most_common << position;
        epsilon_rate += least_common << position;
    }

    gamma_rate * epsilon_rate
}

fn life_support_rating(diagnostic_report: Vec<u32>) -> u32 {
    let mut temp_report = diagnostic_report;
    for position in (0..binary::max_bits_length(&temp_report)).rev() {
        let most_common = binary::most_common_bit_at(position, &temp_report.clone());
        temp_report = temp_report
            .into_iter()
            .filter(|&number| {
                let bit = number >> position & 1;
                bit == most_common
            })
            .collect();
    }
    *temp_report.first().unwrap()
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

    #[test]
    fn sample_input_life_support_rating() {
        let diagnostic_report: Vec<u32> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        assert_eq!(life_support_rating(diagnostic_report), 230);
    }
}
