use crate::parser;

pub fn part1() -> usize {
    let numbers: Vec<isize> = parser::read("data/day20.txt").unwrap();
    sum_after_mixing(&numbers)
}

pub fn part2() -> usize {
    0
}

fn sum_after_mixing(numbers: &Vec<isize>) -> usize {
    let mut rotated = numbers.clone();

    for number in numbers {
        let pos = rotated.iter().position(|n| n == number).unwrap();
        let mut ns = pos as isize + number;
        if ns <= 0 {
            ns -= 1;
        }
        if ns >= numbers.len() as isize {
            ns += 1;
        }
        ns = ns.rem_euclid(numbers.len() as isize);

        rotated.remove(pos);
        rotated.insert(ns as usize, *number);

        // println!("{:?}", blah);
    }

    let zero_pos = rotated.iter().position(|n| *n == 0).unwrap();
    let a = (zero_pos + 1000).rem_euclid(numbers.len());
    let b = (zero_pos + 2000).rem_euclid(numbers.len());
    let c = (zero_pos + 3000).rem_euclid(numbers.len());

    (rotated[a] + rotated[b] + rotated[c]) as usize
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
