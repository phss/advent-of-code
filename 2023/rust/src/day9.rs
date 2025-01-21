use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day9.txt").unwrap();
    let histories = parse(&lines);
    sum_of_extrapolated(&histories)
}

pub fn part2() -> usize {
    0
}

fn sum_of_extrapolated(histories: &Vec<Vec<isize>>) -> usize {
    histories
        .iter()
        .map(extrapolate)
        .reduce(|acc, n| acc + n)
        .unwrap() as usize
}

fn extrapolate(history: &Vec<isize>) -> isize {
    let mut lasts = Vec::new();
    let mut levels = history.clone();

    while !levels.iter().all(|n| *n == 0) {
        lasts.push(levels.iter().last().unwrap().clone());

        levels = levels.windows(2).map(|ns| ns[1] - ns[0]).collect();
    }

    let mut extrapolated = 0;
    while let Some(number) = lasts.pop() {
        extrapolated += number;
    }

    extrapolated
}

fn parse(lines: &Vec<String>) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let histories = parse(&lines);

        let result = sum_of_extrapolated(&histories);

        assert_eq!(result, 114);
    }

    #[test]
    fn sample_input_part_2() {}
}
