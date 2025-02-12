mod module;
mod simulation;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day20.txt").unwrap();
    let mut sim = simulation::Simulation::parse(&lines);
    pulse_mults(&mut sim)
}

pub fn part2() -> usize {
    0
}

fn pulse_mults(sim: &mut simulation::Simulation) -> usize {
    for _ in 0..1000 {
        sim.press_button();
    }
    let high_pulses = sim.pulses.iter().filter(|pulse| pulse.on).count();
    let low_pulses = sim.pulses.iter().filter(|pulse| !pulse.on).count();

    high_pulses * low_pulses
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

        let result = pulse_mults(&mut sim);

        assert_eq!(result, 32000000);
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
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let mut sim = simulation::Simulation::parse(&lines);

        let result = pulse_mults(&mut sim);

        assert_eq!(result, 11687500);
    }
}
