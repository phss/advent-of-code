use crate::parser;

pub fn part1() -> usize {
    let numbers: Vec<isize> = parser::read("data/day20.txt").unwrap();
    sum_after_mixing(&isize)
}

pub fn part2() -> usize {
    0
}

fn sum_after_mixing(numbers: &Vec<isize>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec!["1", "2", "-3", "3", "-2", "0", "4"];
        let numbers: Vec<isize> = input.iter().map(|s| s.parse().unwrap()).collect();

        let result = sum_after_mixing(&numbers);

        assert_eq!(result, 3);
    }

    #[test]
    fn sample_input_part_2() {}
}
