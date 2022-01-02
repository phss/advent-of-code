use crate::parser;

pub fn part1() -> u32 {
    let measurements = parser::read("data/day1.txt").unwrap();
    calculate_increases(measurements)
}

pub fn part2() -> u32 {
    let mut measurements = parser::read("data/day1.txt").unwrap();
    measurements = sliding_windows_sums(&measurements);
    calculate_increases(measurements)
}

fn sliding_windows_sums(measurements: &Vec<u32>) -> Vec<u32> {
    measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

fn calculate_increases(measurements: Vec<u32>) -> u32 {
    measurements
        .windows(2)
        .filter(|pair| matches!(pair, [a, b] if b > a))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increases_in_sample_input() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(calculate_increases(measurements), 7);
    }

    #[test]
    fn only_increases() {
        let measurements = vec![1, 2, 3];
        assert_eq!(calculate_increases(measurements), 2);
    }

    #[test]
    fn no_increases() {
        let measurements = vec![3, 2, 1];
        assert_eq!(calculate_increases(measurements), 0);
    }

    #[test]
    fn sliding_windows_measurements() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(
            sliding_windows_sums(&measurements),
            vec![607, 618, 618, 617, 647, 716, 769, 792]
        );
    }
}
