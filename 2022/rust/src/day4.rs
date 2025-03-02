use std::str::FromStr;

use crate::parser;

struct Section(usize, usize);

struct ElfPair(Section, Section);

#[derive(Debug, Clone)]
struct ParseElfPairError;

impl FromStr for ElfPair {
    type Err = ParseElfPairError;

    fn from_str(s: &str) -> Result<Self, ParseElfPairError> {
        let (ab, cd) = s.split_once(',').unwrap();

        let (a, b) = ab.split_once('-').unwrap();
        let ab = Section(a.parse().unwrap(), b.parse().unwrap());

        let (c, d) = cd.split_once('-').unwrap();
        let cd = Section(c.parse().unwrap(), d.parse().unwrap());

        Ok(ElfPair(ab, cd))
    }
}

pub fn part1() -> usize {
    let pairs: Vec<ElfPair> = parser::read("data/day4.txt").unwrap();
    overlapping_pairs(&pairs)
}

pub fn part2() -> usize {
    let pairs: Vec<ElfPair> = parser::read("data/day4.txt").unwrap();
    any_overlap_pairs(&pairs)
}

fn overlapping_pairs(pairs: &Vec<ElfPair>) -> usize {
    pairs
        .iter()
        .filter(|pair| is_total_overlap(pair))
        .count()
        .try_into()
        .unwrap()
}

fn is_total_overlap(pair: &ElfPair) -> bool {
    let ElfPair(Section(a, b), Section(c, d)) = pair;

    (a <= c && b >= d) || (c <= a && d >= b)
}

fn any_overlap_pairs(pairs: &Vec<ElfPair>) -> usize {
    pairs
        .iter()
        .filter(|pair| is_any_overlap(pair))
        .count()
        .try_into()
        .unwrap()
}

fn is_any_overlap(pair: &ElfPair) -> bool {
    let ElfPair(Section(a, b), Section(c, d)) = pair;

    (a <= c && b >= c) || (a <= d && b >= d) || (c <= a && d >= a) || (c <= b && d >= b) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_overlapping_pairs() {
        let pairs = vec![
            ElfPair(Section(2, 4), Section(6, 8)),
            ElfPair(Section(2, 3), Section(4, 5)),
            ElfPair(Section(5, 7), Section(7, 9)),
            ElfPair(Section(2, 8), Section(3, 7)),
            ElfPair(Section(6, 6), Section(4, 6)),
            ElfPair(Section(2, 6), Section(4, 8)),
        ];
        assert_eq!(overlapping_pairs(&pairs), 2);
    }

    #[test]
    fn sample_input_any_overlap_pairs() {
        let pairs = vec![
            ElfPair(Section(2, 4), Section(6, 8)),
            ElfPair(Section(2, 3), Section(4, 5)),
            ElfPair(Section(5, 7), Section(7, 9)),
            ElfPair(Section(2, 8), Section(3, 7)),
            ElfPair(Section(6, 6), Section(4, 6)),
            ElfPair(Section(2, 6), Section(4, 8)),
        ];
        assert_eq!(any_overlap_pairs(&pairs), 4);
    }
}
