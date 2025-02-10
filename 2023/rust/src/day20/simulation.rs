use std::collections::HashMap;

use super::module::{Broadcaster, Conjunction, FlipFlop};

#[derive(Debug)]
pub struct Simulation {
    pub broadcaster: Broadcaster,
    pub flip_flops: HashMap<String, FlipFlop>,
    pub conjunctions: HashMap<String, Conjunction>,
}

impl Simulation {
    pub fn parse(lines: &Vec<String>) -> Self {
        let broadcaster: Broadcaster = lines
            .iter()
            .find(|line| line.starts_with("broadcaster"))
            .map(|line| line.parse().unwrap())
            .unwrap();

        let flip_flops: HashMap<String, FlipFlop> = lines
            .iter()
            .filter(|line| line.starts_with("%"))
            .map(|line| {
                let flip_flop: FlipFlop = line.parse().unwrap();
                (flip_flop.label.clone(), flip_flop)
            })
            .collect();

        let conjunctions: HashMap<String, Conjunction> = lines
            .iter()
            .filter(|line| line.starts_with("&"))
            .map(|line| {
                let conjunction: Conjunction = line.parse().unwrap();
                (conjunction.label.clone(), conjunction)
            })
            .collect();

        let mut inputs = HashMap::new();
        broadcaster.destinations.iter().for_each(|d| {
            inputs
                .entry(d)
                .or_insert(Vec::new())
                .push("broadcaster".to_string())
        });
        for flip_flop in flip_flops.values() {
            flip_flop.destinations.iter().for_each(|d| {
                inputs
                    .entry(d)
                    .or_insert(Vec::new())
                    .push(flip_flop.label.clone())
            });
        }
        for conjunction in conjunctions.values() {
            conjunction.destinations.iter().for_each(|d| {
                inputs
                    .entry(d)
                    .or_insert(Vec::new())
                    .push(conjunction.label.clone())
            });
        }

        let mut conjunctions: HashMap<String, Conjunction> = conjunctions
            .iter()
            .clone()
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect();

        for conjunction in conjunctions.values_mut() {
            for blah in inputs.get(&conjunction.label).unwrap_or(&Vec::new()) {
                conjunction.input_pulses.insert(blah.clone(), false);
            }
        }

        Simulation {
            broadcaster,
            flip_flops,
            conjunctions,
        }
    }
}
