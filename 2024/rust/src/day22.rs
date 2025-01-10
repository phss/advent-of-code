use std::ops::BitXor;

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day22.txt").unwrap();
    let mut secret_numbers = lines.iter().map(|s| s.parse().unwrap()).collect();
    let result = sum_of_2000th_secret_numbers(&mut secret_numbers);
    println!("Result {result}");
    0
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day22.txt").unwrap();
    let secret_numbers = lines.iter().map(|s| s.parse().unwrap()).collect();
    let result = most_bananas(&secret_numbers);
    println!("Result {result}");
    result as u32
}

fn sum_of_2000th_secret_numbers(secret_numbers: &mut Vec<usize>) -> usize {
    for _ in 0..2000 {
        secret_numbers
            .iter_mut()
            .for_each(|num| *num = evolve(*num));
    }

    secret_numbers.iter().sum()
}

fn most_bananas(secret_numbers: &Vec<usize>) -> usize {
    0
}

fn evolve(secret_number: usize) -> usize {
    let mut secret_number = prune(mix(secret_number * 64, secret_number));
    secret_number = prune(mix(secret_number / 32, secret_number));
    prune(mix(secret_number * 2048, secret_number))
}

fn mix(a: usize, b: usize) -> usize {
    a.bitxor(b)
}

fn prune(a: usize) -> usize {
    a.rem_euclid(16777216)
}

fn price(a: usize) -> usize {
    a % 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let mut secret_numbers = vec![1, 10, 100, 2024];

        let result = sum_of_2000th_secret_numbers(&mut secret_numbers);

        assert_eq!(result, 37327623);
    }

    #[test]
    fn sample_input_part_2() {
        let secret_numbers = vec![1, 10, 100, 2024];

        let result = most_bananas(&secret_numbers);

        assert_eq!(result, 23);
    }

    #[test]
    fn test_evolve() {
        let mut secret_number = 123;
        let expected_evolution = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for expected in expected_evolution {
            let evolved = evolve(secret_number);
            assert_eq!(evolved, expected);
            secret_number = evolved;
        }
    }

    #[test]
    fn test_price() {
        let mut secret_number = 123;
        let expected_evolution = vec![0, 6, 5, 4, 4, 6, 4, 4, 2];

        for expected in expected_evolution {
            let evolved = evolve(secret_number);
            let price = price(evolved);
            assert_eq!(price, expected);
            secret_number = evolved;
        }
    }
}
