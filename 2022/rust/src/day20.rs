use crate::parser;

pub fn part1() -> usize {
    let numbers: Vec<isize> = parser::read("data/day20.txt").unwrap();
    sum_after_mixing(&numbers, 1, 1)
}

pub fn part2() -> usize {
    let numbers: Vec<isize> = parser::read("data/day20.txt").unwrap();
    sum_after_mixing(&numbers, 811589153, 10)
}

fn sum_after_mixing(numbers: &Vec<isize>, decryption_key: usize, times: usize) -> usize {
    let mut mixed: Vec<(usize, isize)> = numbers
        .iter()
        .map(|n| n * decryption_key as isize)
        .enumerate()
        .clone()
        .collect();

    for _ in 0..times {
        for original_index in 0..mixed.len() {
            let mix_from = mixed
                .iter()
                .position(|(i, _)| *i == original_index)
                .unwrap();

            let elem = mixed.remove(mix_from);

            let mut mix_to = mix_from as isize + elem.1;
            mix_to = mix_to.rem_euclid(mixed.len() as isize);

            mixed.insert(mix_to as usize, elem);
        }
    }

    let zero_pos = mixed.iter().position(|(_, n)| *n == 0).unwrap();
    let a = (zero_pos + 1000).rem_euclid(mixed.len());
    let b = (zero_pos + 2000).rem_euclid(mixed.len());
    let c = (zero_pos + 3000).rem_euclid(mixed.len());

    (mixed[a].1 + mixed[b].1 + mixed[c].1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec!["1", "2", "-3", "3", "-2", "0", "4"];
        let numbers: Vec<isize> = input.iter().map(|s| s.parse().unwrap()).collect();

        let result = sum_after_mixing(&numbers, 1, 1);

        assert_eq!(result, 3);
    }

    #[test]
    fn sample_input_part_2() {
        let input = vec!["1", "2", "-3", "3", "-2", "0", "4"];
        let numbers: Vec<isize> = input.iter().map(|s| s.parse().unwrap()).collect();

        let result = sum_after_mixing(&numbers, 811589153, 10);

        assert_eq!(result, 1623178306);
    }
}
