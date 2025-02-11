mod module;
mod simulation;

use crate::parser;

struct Module {}

pub fn part1() -> usize {
    0
}

pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "broadcaster -> a, b, c",
            "%a -> b",
            "%b -> c",
            "%c -> inv",
            "&inv -> a",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let mut sim = simulation::Simulation::parse(&lines);

        sim.press_button();

        for pulse in sim.pulses {
            println!("{:?}", pulse);
        }

        // let result = pulses_mut(&workflows, &parts);

        // assert_eq!(result, 32000000);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ];
        // let modules: Vec<Module> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        // println!("{:?}", modules);

        // let result = pulses_mut(&workflows, &parts);

        // assert_eq!(result, 11687500);
    }
}
