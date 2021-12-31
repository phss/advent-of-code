use std::fs;

pub fn part1() -> u32 {
    let measurements = parse("data/day1.txt");
    calculate_increases(measurements)
}

fn parse(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    contents
        .split("\n")
        .map(|line| line.parse().expect("not a number"))
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
    fn sample_input() {
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
}
