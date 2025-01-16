use itertools::Itertools;

use crate::parser;

type Mapping = (usize, usize, usize);

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day5.txt").unwrap();
    let (seeds, all_mappings) = parse(&lines);
    lowest_location(&seeds, &all_mappings)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day5.txt").unwrap();
    let (seeds, all_mappings) = parse(&lines);
    lowest_location_with_range(&seeds, &all_mappings)
}

fn lowest_location(seeds: &Vec<usize>, all_mappings: &Vec<Vec<Mapping>>) -> usize {
    let mut seed_ranges = Vec::new();

    for seed in seeds {
        seed_ranges.push(*seed);
        seed_ranges.push(1);
    }

    lowest_location_with_range(&seed_ranges, all_mappings)
}

fn lowest_location_with_range(seed_ranges: &Vec<usize>, all_mappings: &Vec<Vec<Mapping>>) -> usize {
    let mut ranges = Vec::new();

    for chunk in seed_ranges.chunks(2) {
        let start = chunk[0];
        let length = chunk[1];
        ranges.push((start, start + length - 1));
    }

    let mut all_sorted_mappings = Vec::new();
    for mappings in all_mappings {
        let sorted_mappings: Vec<Mapping> = mappings
            .clone()
            .into_iter()
            .sorted_by_key(|(_, src, _)| *src)
            .collect();

        let mut last_end = 0;
        let mut new_sorted_mappings = Vec::new();
        for mapping @ (_, src, length) in sorted_mappings {
            if src > last_end {
                new_sorted_mappings.push((last_end, last_end, src - last_end));
            }

            new_sorted_mappings.push(mapping);
            last_end = src + length;
        }
        new_sorted_mappings.push((last_end, last_end, usize::MAX - last_end));

        all_sorted_mappings.push(new_sorted_mappings);
    }

    lowest_with_ranges(&ranges, &all_sorted_mappings)
}

fn lowest_with_ranges(ranges: &Vec<(usize, usize)>, all_mappings: &Vec<Vec<Mapping>>) -> usize {
    if all_mappings.is_empty() {
        return *ranges.iter().map(|(low, _)| low).min().unwrap();
    }

    let (mappings, rest) = all_mappings.split_at(1);
    let mappings = &mappings[0];
    let rest = rest.to_vec();
    let mut new_ranges = Vec::new();

    for (range_start, range_end) in ranges {
        for (dst, src, length) in mappings {
            let mapping_start = src;
            let mapping_end = src + length - 1;

            if *range_start <= mapping_end && range_end >= mapping_start {
                let overlap_start = range_start.max(mapping_start);
                let overlap_end = range_end.min(&mapping_end);

                let new_range_start = overlap_start - mapping_start + dst;
                let new_range_end = overlap_end - mapping_start + dst;

                new_ranges.push((new_range_start, new_range_end));
            }
        }
    }

    lowest_with_ranges(&new_ranges, &rest)
}

fn parse(lines: &Vec<String>) -> (Vec<usize>, Vec<Vec<Mapping>>) {
    let mut parts = lines.split(String::is_empty);

    let seeds = parts
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut all_mappings = Vec::new();
    for part in parts {
        let mappings = part
            .iter()
            .skip(1)
            .map(|m| {
                let elements: Vec<usize> = m.split(" ").map(|s| s.parse().unwrap()).collect();
                (elements[0], elements[1], elements[2])
            })
            .collect();
        all_mappings.push(mappings);
    }

    (seeds, all_mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let (seeds, all_mappings) = parse(&lines);

        let result = lowest_location(&seeds, &all_mappings);

        assert_eq!(result, 35)
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let (seeds, all_mappings) = parse(&lines);

        let result = lowest_location_with_range(&seeds, &all_mappings);

        assert_eq!(result, 46)
    }
}
