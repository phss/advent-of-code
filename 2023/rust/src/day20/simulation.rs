use super::module::{Broadcaster, Conjunction, FlipFlop};

#[derive(Debug)]
pub struct Simulation {
    pub broadcaster: Broadcaster,
    pub flip_flops: Vec<FlipFlop>,
    pub conjunctions: Vec<Conjunction>,
}

impl Simulation {
    pub fn parse(lines: &Vec<String>) -> Self {
        let broadcaster = lines
            .iter()
            .find(|line| line.starts_with("broadcaster"))
            .map(|line| line.parse().unwrap())
            .unwrap();

        let flip_flops = lines
            .iter()
            .filter(|line| line.starts_with("%"))
            .map(|line| line.parse().unwrap())
            .collect();

        let conjunctions = lines
            .iter()
            .filter(|line| line.starts_with("&"))
            .map(|line| line.parse().unwrap())
            .collect();

        Simulation {
            broadcaster,
            flip_flops,
            conjunctions,
        }
    }
}
