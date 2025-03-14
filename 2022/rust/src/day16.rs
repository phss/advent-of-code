use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

use crate::parser;

#[derive(Debug, Clone, PartialEq)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct ParseValveErr;

impl FromStr for Valve {
    type Err = ParseValveErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_line = s.split("; ");

        let part = splitted_line.next().unwrap();
        let mut blah = part.split_ascii_whitespace();
        let name = blah.nth(1).unwrap().to_string();
        let flow_rate: usize = blah.nth(2).unwrap()[5..].parse().unwrap();

        let part = splitted_line.next().unwrap();
        let ti = if part.starts_with("tunnels") { 23 } else { 22 };
        let tunnels: Vec<String> = part[ti..].split(", ").map(|s| s.to_string()).collect();

        Ok(Self {
            name,
            flow_rate,
            tunnels,
        })
    }
}

pub fn part1() -> usize {
    let valves: Vec<Valve> = parser::read("data/day16.txt").unwrap();
    most_pressure_released(&valves)
}

pub fn part2() -> usize {
    let valves: Vec<Valve> = parser::read("data/day16.txt").unwrap();
    most_pressure_released_with_elephant(&valves)
}

fn most_pressure_released(valves: &Vec<Valve>) -> usize {
    all_valid_paths(valves, 30)
        .iter()
        .map(|p| p.1)
        .max()
        .unwrap() as usize
}

fn most_pressure_released_with_elephant(valves: &Vec<Valve>) -> usize {
    let mut max_pressure = 0;
    let paths = all_valid_paths(valves, 26);
    let valid_paths: Vec<&(HashSet<String>, usize)> =
        paths.iter().sorted_by_key(|(_, flow)| flow).rev().collect();

    for i in 0..valid_paths.len() {
        for j in i..valid_paths.len() {
            let (a_valves, a_flow) = valid_paths[i];
            let (b_valves, b_flow) = valid_paths[j];

            let pressure = a_flow + b_flow;
            if pressure < max_pressure {
                break;
            }

            if a_valves.iter().all(|v| !b_valves.contains(v)) && pressure > max_pressure {
                max_pressure = pressure;
            }
        }
    }

    max_pressure
}

fn all_valid_paths(valves: &Vec<Valve>, max_minutes: usize) -> Vec<(HashSet<String>, usize)> {
    let mut all_paths = vec![];
    let distances = distances_between_valves(&valves);

    let mut valve_with_flow_lookup = vec![];
    for valve in valves {
        if valve.flow_rate > 0 {
            valve_with_flow_lookup.push((valve.name.clone(), valve.flow_rate));
        }
    }

    let mut pressures: Vec<(String, HashSet<String>, usize, usize)> =
        vec![("AA".to_string(), HashSet::new(), 0, 0)];

    while pressures.len() > 0 {
        let (current_valve, visited_valves, minutes, pressure) = pressures.pop().unwrap();
        all_paths.push((visited_valves.clone(), pressure));

        for (valve, flow_rate) in &valve_with_flow_lookup {
            if !visited_valves.contains(valve) {
                let valve_open_minutes = minutes + distances[&current_valve][valve] + 1;

                if valve_open_minutes < max_minutes {
                    let new_pressure = pressure + flow_rate * (max_minutes - valve_open_minutes);
                    let mut next_visited = visited_valves.clone();
                    next_visited.insert(valve.clone());
                    pressures.push((
                        valve.clone(),
                        next_visited,
                        valve_open_minutes,
                        new_pressure,
                    ));
                }
            }
        }
    }

    all_paths
}

fn distances_between_valves(valves: &Vec<Valve>) -> HashMap<String, HashMap<String, usize>> {
    let mut distances = HashMap::new();

    let mut valve_lookup = HashMap::new();
    for valve in valves {
        valve_lookup.insert(valve.name.clone(), valve);
    }

    for valve in valves {
        let mut valve_distances = HashMap::new();
        let mut visited_valves = HashSet::new();
        let mut next_valves = VecDeque::new();

        visited_valves.insert(valve.name.clone());
        next_valves.push_back((valve.name.clone(), 0));

        while visited_valves.len() < valves.len() {
            let (current_valve, current_distance) = next_valves.pop_front().unwrap();

            for potential_next_valve in &valve_lookup.get(&current_valve).unwrap().tunnels {
                if !visited_valves.contains(potential_next_valve) {
                    let valve_distance = current_distance + 1;
                    valve_distances.insert(potential_next_valve.clone(), valve_distance);
                    visited_valves.insert(potential_next_valve.clone());
                    next_valves.push_back((potential_next_valve.clone(), valve_distance));
                }
            }
        }

        distances.insert(valve.name.clone(), valve_distances);
    }

    distances
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            Valve::from_str("Valve AA has flow rate=42; tunnels lead to valves DD, II, BB"),
            Ok(Valve {
                name: String::from("AA"),
                flow_rate: 42,
                tunnels: vec![String::from("DD"), String::from("II"), String::from("BB")],
            })
        );
        assert_eq!(
            Valve::from_str("Valve AA has flow rate=42; tunnel leads to valve DD"),
            Ok(Valve {
                name: String::from("AA"),
                flow_rate: 42,
                tunnels: vec![String::from("DD")],
            })
        );
    }

    #[test]
    fn distances() {
        let valves: Vec<Valve> = vec![
            String::from("Valve AA has flow rate=0; tunnels lead to valves BB, DD"),
            String::from("Valve BB has flow rate=13; tunnels lead to valves AA, CC, DD"),
            String::from("Valve CC has flow rate=2; tunnels lead to valves BB, EE"),
            String::from("Valve DD has flow rate=20; tunnels lead to valves AA, BB"),
            String::from("Valve EE has flow rate=20; tunnel leads to valve CC"),
        ]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

        let mut expected_distances: HashMap<String, HashMap<String, usize>> = HashMap::new();

        let mut dists = HashMap::new();
        dists.insert("BB".to_string(), 1);
        dists.insert("CC".to_string(), 2);
        dists.insert("DD".to_string(), 1);
        dists.insert("EE".to_string(), 3);
        expected_distances.insert("AA".to_string(), dists);

        let mut dists = HashMap::new();
        dists.insert("AA".to_string(), 1);
        dists.insert("CC".to_string(), 1);
        dists.insert("DD".to_string(), 1);
        dists.insert("EE".to_string(), 2);
        expected_distances.insert("BB".to_string(), dists);

        let mut dists = HashMap::new();
        dists.insert("AA".to_string(), 2);
        dists.insert("BB".to_string(), 1);
        dists.insert("DD".to_string(), 2);
        dists.insert("EE".to_string(), 1);
        expected_distances.insert("CC".to_string(), dists);

        let mut dists = HashMap::new();
        dists.insert("AA".to_string(), 1);
        dists.insert("BB".to_string(), 1);
        dists.insert("CC".to_string(), 2);
        dists.insert("EE".to_string(), 3);
        expected_distances.insert("DD".to_string(), dists);

        let mut dists = HashMap::new();
        dists.insert("AA".to_string(), 3);
        dists.insert("BB".to_string(), 2);
        dists.insert("CC".to_string(), 1);
        dists.insert("DD".to_string(), 3);
        expected_distances.insert("EE".to_string(), dists);

        assert_eq!(distances_between_valves(&valves), expected_distances);
    }

    #[test]
    fn sample_input_part_1() {
        let valves: Vec<Valve> = vec![
            String::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            String::from("Valve BB has flow rate=13; tunnels lead to valves CC, AA"),
            String::from("Valve CC has flow rate=2; tunnels lead to valves DD, BB"),
            String::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE"),
            String::from("Valve EE has flow rate=3; tunnels lead to valves FF, DD"),
            String::from("Valve FF has flow rate=0; tunnels lead to valves EE, GG"),
            String::from("Valve GG has flow rate=0; tunnels lead to valves FF, HH"),
            String::from("Valve HH has flow rate=22; tunnel leads to valve GG"),
            String::from("Valve II has flow rate=0; tunnels lead to valves AA, JJ"),
            String::from("Valve JJ has flow rate=21; tunnel leads to valve II"),
        ]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
        assert_eq!(most_pressure_released(&valves), 1651);
    }

    #[test]
    fn sample_input_part_2() {
        let valves: Vec<Valve> = vec![
            String::from("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            String::from("Valve BB has flow rate=13; tunnels lead to valves CC, AA"),
            String::from("Valve CC has flow rate=2; tunnels lead to valves DD, BB"),
            String::from("Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE"),
            String::from("Valve EE has flow rate=3; tunnels lead to valves FF, DD"),
            String::from("Valve FF has flow rate=0; tunnels lead to valves EE, GG"),
            String::from("Valve GG has flow rate=0; tunnels lead to valves FF, HH"),
            String::from("Valve HH has flow rate=22; tunnel leads to valve GG"),
            String::from("Valve II has flow rate=0; tunnels lead to valves AA, JJ"),
            String::from("Valve JJ has flow rate=21; tunnel leads to valve II"),
        ]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
        assert_eq!(most_pressure_released_with_elephant(&valves), 1707);
    }
}
