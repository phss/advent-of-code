use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use super::module::{Broadcaster, Conjunction, FlipFlop, Module};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pulse {
    pub from: String,
    pub to: String,
    pub on: bool,
}

pub struct Simulation {
    pub pulses: Vec<Pulse>,
    pub modules: HashMap<String, Box<dyn Module>>,
}

impl Simulation {
    pub fn press_button(&mut self) {
        let mut to_process = VecDeque::new();
        to_process.push_back(Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            on: false,
        });

        while let Some(pulse) = to_process.pop_front() {
            self.pulses.push(pulse.clone());

            let module = self.modules.get_mut(&pulse.to).unwrap();
            for output_pulse in module.process(pulse) {
                to_process.push_back(output_pulse);
            }
        }
    }

    pub fn parse(lines: &Vec<String>) -> Self {
        let broadcaster: Broadcaster = Simulation::filter_and_parse(lines, "broadcaster")
            .next()
            .unwrap();

        let flip_flops: Vec<FlipFlop> = Simulation::filter_and_parse(lines, "%").collect();

        let conjunctions: Vec<Conjunction> = Simulation::filter_and_parse(lines, "&").collect();

        let mut inputs = HashMap::new();
        broadcaster.destinations.iter().for_each(|d| {
            inputs
                .entry(d)
                .or_insert(Vec::new())
                .push("broadcaster".to_string())
        });
        for flip_flop in &flip_flops {
            flip_flop.destinations.iter().for_each(|d| {
                inputs
                    .entry(d)
                    .or_insert(Vec::new())
                    .push(flip_flop.label.clone())
            });
        }
        for conjunction in &conjunctions {
            conjunction.destinations.iter().for_each(|d| {
                inputs
                    .entry(d)
                    .or_insert(Vec::new())
                    .push(conjunction.label.clone())
            });
        }

        let mut conjunctions: Vec<Conjunction> = conjunctions.iter().cloned().collect();
        for conjunction in conjunctions.iter_mut() {
            for destination in inputs.get(&conjunction.label).unwrap_or(&Vec::new()) {
                conjunction.input_pulses.insert(destination.clone(), false);
            }
        }

        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        modules.insert("broadcaster".to_string(), Box::new(broadcaster));
        for flip_flop in flip_flops {
            modules.insert(flip_flop.label.clone(), Box::new(flip_flop));
        }
        for conjunction in conjunctions {
            modules.insert(conjunction.label.clone(), Box::new(conjunction));
        }

        Simulation {
            pulses: Vec::new(),
            modules,
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
