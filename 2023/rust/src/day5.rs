use std::collections::HashSet;

use crate::parser;

type Mapping = (usize, usize, usize);

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day5.txt").unwrap();
    let (seeds, all_mappings) = parse(&lines);
    lowest_location(&seeds, &all_mappings) as u32
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day5.txt").unwrap();
    let (seeds, all_mappings) = parse(&lines);
    let result = lowest_location_with_range(&seeds, &all_mappings);
    println!("Result {:?}", result);
    result as u32
}

fn lowest_location(seeds: &Vec<usize>, all_mappings: &Vec<Vec<Mapping>>) -> usize {
    let mut locations = seeds.clone();

    for mappings in all_mappings {
        locations = locations
            .iter()
            .map(|value| {
                mappings
                    .iter()
                    .find(|(_, src, len)| value >= src && *value <= (src + len))
                    .map(|(dst, src, _)| dst + *value - src)
                    .unwrap_or(*value)
            })
            .collect();
    }

    *locations.iter().min().unwrap()
}

fn lowest_location_with_range(seed_ranges: &Vec<usize>, all_mappings: &Vec<Vec<Mapping>>) -> usize {
    let mut seeds = Vec::new();

    for seed_range in seed_ranges.chunks(2) {
        let start = seed_range[0];
        let length = seed_range[1];

        seeds.extend(start..start + length);
    }

    lowest_location(&seeds, all_mappings)
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
