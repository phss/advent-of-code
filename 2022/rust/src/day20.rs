use crate::parser;

pub fn part1() -> usize {
    let numbers: Vec<isize> = parser::read("data/day20.txt").unwrap();
    sum_after_mixing(&numbers)
}

pub fn part2() -> usize {
    0
}

fn sum_after_mixing(numbers: &Vec<isize>) -> usize {
    let mut mixed: Vec<(usize, &isize)> = numbers.iter().enumerate().clone().collect();

    for (original_index, number) in numbers.iter().enumerate() {
        let mix_from = mixed
            .iter()
            .position(|(i, _)| *i == original_index)
            .unwrap();

        let mut mix_to = mix_from as isize + number;
        mix_to = mix_to.rem_euclid(mixed.len() as isize - 1);

        let elem = mixed.remove(mix_from);
        mixed.insert(mix_to as usize, elem);
    }

    let zero_pos = mixed.iter().position(|(_, n)| **n == 0).unwrap();
    let a = (zero_pos + 1000).rem_euclid(numbers.len());
    let b = (zero_pos + 2000).rem_euclid(numbers.len());
    let c = (zero_pos + 3000).rem_euclid(numbers.len());

    (mixed[a].1 + mixed[b].1 + mixed[c].1) as usize
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
