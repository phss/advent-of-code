mod module;
mod simulation;

use std::collections::HashSet;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day20.txt").unwrap();
    let mut sim = simulation::Simulation::parse(&lines);
    pulse_mults(&mut sim)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day20.txt").unwrap();
    let mut sim = simulation::Simulation::parse(&lines);
    presses_until_rx(&mut sim)
}

fn pulse_mults(sim: &mut simulation::Simulation) -> usize {
    for _ in 0..1000 {
        sim.press_button();
    }
    let high_pulses = sim.pulses.iter().filter(|pulse| pulse.on).count();
    let low_pulses = sim.pulses.iter().filter(|pulse| !pulse.on).count();

    high_pulses * low_pulses
}

fn presses_until_rx(sim: &mut simulation::Simulation) -> usize {
    let mut presses = 0;
    let mut remaining_fan_in_nodes = HashSet::from([
        "qz".to_string(),
        "cq".to_string(),
        "jx".to_string(),
        "tt".to_string(),
    ]);
    let mut activation_presses = Vec::new();

    while !remaining_fan_in_nodes.is_empty() {
        presses += 1;
        sim.press_button();

        if let Some(pulse) = sim.pulses.iter().find(|pulse| {
            pulse.on && pulse.to == "qn" && remaining_fan_in_nodes.contains(&pulse.from)
        }) {
            activation_presses.push(presses);
            remaining_fan_in_nodes.remove(&pulse.from);
        }

        sim.pulses.clear();
    }

    lcm(&activation_presses)
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
