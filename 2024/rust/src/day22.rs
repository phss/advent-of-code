use std::{
    collections::{HashMap, HashSet},
    ops::BitXor,
};

use itertools::Itertools;

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
    most_bananas(&secret_numbers) as u32
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
    let mut bananas_per_changes = HashMap::new();

    for initial in secret_numbers {
        let (prices, price_changes) = price_analysis(*initial, 2001);
        let mut ocurred = HashSet::new();

        for (i, blah) in price_changes.windows(4).enumerate() {
            let change_set = blah.into_iter().cloned().collect_vec();
            if ocurred.contains(&change_set) {
                continue;
            }

            let price = prices[i + 4];
            *bananas_per_changes.entry(change_set.clone()).or_insert(0) += price;
            ocurred.insert(change_set);
        }
    }

    *bananas_per_changes.values().max().unwrap()
}

fn price_analysis(initial: usize, times: usize) -> (Vec<usize>, Vec<isize>) {
    let mut prices = Vec::new();
    let mut price_changes = Vec::new();
    let mut secret_number = initial;

    prices.push(price(secret_number));
    for i in 1..times {
        secret_number = evolve(secret_number);
        prices.push(price(secret_number));
        price_changes.push(prices[i] as isize - prices[i - 1] as isize);
    }

    (prices, price_changes)
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
        let secret_numbers = vec![1, 2, 3, 2024];

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
    fn test_price_analysis() {
        let (prices, price_changes) = price_analysis(123, 10);

        assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
        assert_eq!(price_changes, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }
}
