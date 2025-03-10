use regex::Regex;
use std::str::FromStr;

use crate::parser;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let caps = re.captures(s).unwrap();

        Ok(Blueprint {
            id: caps[1].parse().unwrap(),
            ore_robot_cost: caps[2].parse().unwrap(),
            clay_robot_cost: caps[3].parse().unwrap(),
            obsidian_robot_cost: (caps[4].parse().unwrap(), caps[5].parse().unwrap()),
            geode_robot_cost: (caps[6].parse().unwrap(), caps[7].parse().unwrap()),
        })
    }
}

pub fn part1() -> usize {
    let blueprints: Vec<Blueprint> = parser::read("data/day19.txt").unwrap();
    sum_quality_levels(&blueprints)
}

pub fn part2() -> usize {
    0
}

fn sum_quality_levels(blueprints: &Vec<Blueprint>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
            "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        ];
        let blueprints: Vec<Blueprint> = input.iter().map(|s| s.parse().unwrap()).collect();

        let result = sum_quality_levels(&blueprints);

        assert_eq!(result, 33);
    }

    #[test]
    fn sample_input_part_2() {}
}
