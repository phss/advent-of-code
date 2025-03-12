use regex::Regex;
use std::{collections::HashSet, str::FromStr};

use crate::parser;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Simulation {
    blueprint: Blueprint,
    materials: (usize, usize, usize, usize),
    robots: (usize, usize, usize, usize),
}

impl Simulation {
    fn from(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            materials: (0, 0, 0, 0),
            robots: (1, 0, 0, 0),
        }
    }

    fn build_ore_robot(&self) -> Option<Self> {
        let ore_cost = self.blueprint.ore_robot_cost;
        if self.materials.0 >= ore_cost {
            let collected = self.collect();
            let mut materials = collected.materials;
            materials.0 -= ore_cost;
            let mut robots = collected.robots;
            robots.0 += 1;
            Some(Simulation {
                blueprint: collected.blueprint,
                materials,
                robots,
            })
        } else {
            None
        }
    }

    fn build_clay_robot(&self) -> Option<Self> {
        let ore_cost = self.blueprint.clay_robot_cost;
        if self.materials.0 >= ore_cost {
            let collected = self.collect();
            let mut materials = collected.materials;
            materials.0 -= ore_cost;
            let mut robots = collected.robots;
            robots.1 += 1;
            Some(Simulation {
                blueprint: collected.blueprint,
                materials,
                robots,
            })
        } else {
            None
        }
    }

    fn build_obisidian_robot(&self) -> Option<Self> {
        let (ore_cost, clay_cost) = self.blueprint.obsidian_robot_cost;
        if self.materials.0 >= ore_cost && self.materials.1 >= clay_cost {
            let collected = self.collect();
            let mut materials = collected.materials;
            materials.0 -= ore_cost;
            materials.1 -= clay_cost;
            let mut robots = collected.robots;
            robots.2 += 1;
            Some(Simulation {
                blueprint: collected.blueprint,
                materials,
                robots,
            })
        } else {
            None
        }
    }

    fn build_geode_robot(&self) -> Option<Self> {
        let (ore_cost, obsidian_cost) = self.blueprint.geode_robot_cost;
        if self.materials.0 >= ore_cost && self.materials.2 >= obsidian_cost {
            let collected = self.collect();
            let mut materials = collected.materials;
            materials.0 -= ore_cost;
            materials.2 -= obsidian_cost;
            let mut robots = collected.robots;
            robots.3 += 1;
            Some(Simulation {
                blueprint: collected.blueprint,
                materials,
                robots,
            })
        } else {
            None
        }
    }

    fn collect(&self) -> Self {
        Self {
            blueprint: self.blueprint,
            materials: (
                self.materials.0 + self.robots.0,
                self.materials.1 + self.robots.1,
                self.materials.2 + self.robots.2,
                self.materials.3 + self.robots.3,
            ),
            robots: self.robots,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
    blueprints
        .iter()
        .map(|blueprint| {
            let simulation = Simulation::from(blueprint.clone());
            let result = simulate_quality_level(HashSet::from([simulation]), 24);
            result
        })
        .sum()
}

fn simulate_quality_level(simulations: HashSet<Simulation>, remaining_minutes: usize) -> usize {
    if remaining_minutes == 0 {
        return simulations
            .iter()
            .map(|simulation| simulation.blueprint.id * simulation.materials.3)
            .max()
            .unwrap();
    }

    let mut candidates = HashSet::new();
    for simulation in simulations {
        if let Some(next) = simulation.build_geode_robot() {
            candidates.insert(next);
        }
        if let Some(next) = simulation.build_obisidian_robot() {
            candidates.insert(next);
        }
        if let Some(next) = simulation.build_clay_robot() {
            if remaining_minutes > 5 {
                candidates.insert(next);
            }
        }
        if let Some(next) = simulation.build_ore_robot() {
            if remaining_minutes > 15 {
                candidates.insert(next);
            }
        }
        candidates.insert(simulation.collect());
    }

    simulate_quality_level(candidates, remaining_minutes - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        // let input = vec![
        //     "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
        //     "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        // ];
        // let blueprints: Vec<Blueprint> = input.iter().map(|s| s.parse().unwrap()).collect();

        // let result = sum_quality_levels(&blueprints);

        // assert_eq!(result, 33);
    }

    #[test]
    fn sample_input_part_2() {}
}
