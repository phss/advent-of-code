use std::{collections::HashSet, str::FromStr};

use crate::parser;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        let id_and_winning_numbers: Vec<&str> = parts[0].trim().split(':').collect();

        let winning_numbers: Vec<usize> = id_and_winning_numbers[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().map_err(|_| "Invalid number".to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        let card_numbers: Vec<usize> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().map_err(|_| "Invalid number".to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Card {
            winning_numbers,
            card_numbers,
        })
    }
}

pub fn part1() -> u32 {
    let cards: Vec<Card> = parser::read("data/day4.txt").unwrap();
    count_points(&cards) as u32
}

pub fn part2() -> u32 {
    0
}

fn count_points(cards: &Vec<Card>) -> usize {
    let mut count = 0;

    for card in cards {
        let winning: HashSet<usize> = HashSet::from_iter(card.winning_numbers.clone());
        let ours: HashSet<usize> = HashSet::from_iter(card.card_numbers.clone());
        let overlap: u32 = winning.intersection(&ours).count() as u32;

        if overlap > 0 {
            count += 2_usize.pow(overlap - 1);
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];
        let cards: Vec<Card> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_points(&cards);

        assert_eq!(result, 13)
    }

    #[test]
    fn sample_input_part_2() {}
}
