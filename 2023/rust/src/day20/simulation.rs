use std::{
    collections::{binary_heap::Iter, HashMap},
    iter::Filter,
    str::FromStr,
};

use super::module::{Broadcaster, Conjunction, FlipFlop};

#[derive(Debug)]
pub struct Simulation {
    pub broadcaster: Broadcaster,
    pub flip_flops: HashMap<String, FlipFlop>,
    pub conjunctions: HashMap<String, Conjunction>,
}

impl Simulation {
    pub fn parse(lines: &Vec<String>) -> Self {
        let broadcaster: Broadcaster = Simulation::filter_and_parse(lines, "broadcaster")
            .next()
            .unwrap();

        let flip_flops: HashMap<String, FlipFlop> = Simulation::filter_and_parse(lines, "%")
            .map(|flip_flop: FlipFlop| (flip_flop.label.clone(), flip_flop))
            .collect();

        let conjunctions: HashMap<String, Conjunction> = Simulation::filter_and_parse(lines, "&")
            .map(|conjunction: Conjunction| (conjunction.label.clone(), conjunction))
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
            for destination in inputs.get(&conjunction.label).unwrap_or(&Vec::new()) {
                conjunction.input_pulses.insert(destination.clone(), false);
            }
        }

        Simulation {
            broadcaster,
            flip_flops,
            conjunctions,
        }
    }

    fn filter_and_parse<'a, T>(
        lines: &'a Vec<String>,
        prefix: &'a str,
    ) -> impl Iterator<Item = T> + 'a
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        lines
            .iter()
            .filter(move |line| line.starts_with(prefix))
            .map(|line| line.parse().unwrap())
    }
}
