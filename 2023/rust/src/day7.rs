use std::str::FromStr;

use itertools::Itertools;

use crate::parser;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: usize,
    jokered: bool,
}

impl Hand {
    fn hand_type_strength(&self) -> usize {
        let card_groups = self.cards.chars().into_group_map_by(|it| *it);
        let mut different_cards = card_groups.len();
        let mut biggest_card_group = card_groups.values().map(|g| g.len()).max().unwrap();

        if self.jokered {
            biggest_card_group = card_groups
                .iter()
                .filter(|(c, _)| **c != 'J')
                .map(|(_, v)| v)
                .map(|g| g.len())
                .max()
                .unwrap_or(0);
            let jokers = card_groups.get(&'J').map(|g| g.len()).unwrap_or(0);
            if jokers > 0 {
                biggest_card_group += jokers;
                different_cards -= 1;
                different_cards = different_cards.max(1);
            }
        }

        match (different_cards, biggest_card_group) {
            (1, _) => 7,
            (2, 4) => 6,
            (2, 3) => 5,
            (3, 3) => 4,
            (3, 2) => 3,
            (4, _) => 2,
            (5, _) => 1,
            e => panic!("should be unreachable: {:?}, {}", e, self.cards),
        }
    }

    fn cards_strength(&self) -> Vec<usize> {
        self.cards
            .chars()
            .map(|card| match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' if self.jokered => 1,
                'J' => 11,
                'T' => 10,
                c => c.to_string().parse().unwrap(),
            })
            .collect()
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let cards = parts.next().unwrap().to_string();
        let bid = parts.next().unwrap().parse().unwrap();

        Ok(Hand {
            cards,
            bid,
            jokered: false,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type_strength()
            .cmp(&other.hand_type_strength())
            .then(self.cards_strength().cmp(&other.cards_strength()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part1() -> usize {
    let hands: Vec<Hand> = parser::read("data/day7.txt").unwrap();
    total_winnings(&hands)
}

pub fn part2() -> usize {
    let hands: Vec<Hand> = parser::read("data/day7.txt").unwrap();
    total_winnings_with_joker(&hands)
}

fn total_winnings(hands: &Vec<Hand>) -> usize {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn total_winnings_with_joker(hands: &Vec<Hand>) -> usize {
    hands
        .iter()
        .map(|hand| Hand {
            cards: hand.cards.clone(),
            bid: hand.bid,
            jokered: true,
        })
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let hands: Vec<Hand> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = total_winnings(&hands);

        assert_eq!(result, 6440)
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let hands: Vec<Hand> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = total_winnings_with_joker(&hands);

        assert_eq!(result, 5905)
    }
}
