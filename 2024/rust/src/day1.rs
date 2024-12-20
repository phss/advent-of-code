use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day1.txt").unwrap();
    let (mut left_list, mut right_list) = to_lists(&lines);
    total_distance(&mut left_list, &mut right_list)
}

pub fn part2() -> u32 {
    0
}

fn to_lists(lines: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        left_list.push(parts.next().unwrap().parse().unwrap());
        right_list.push(parts.next().unwrap().parse().unwrap());
    }

    (left_list, right_list)
}

fn total_distance(left_list: &mut Vec<u32>, right_list: &mut Vec<u32>) -> u32 {
    left_list.sort();
    right_list.sort();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let mut left_list = vec![3, 4, 2, 1, 3, 3];
        let mut right_list = vec![4, 3, 5, 3, 9, 3];

        let result = total_distance(&mut left_list, &mut right_list);

        assert_eq!(result, 11);
    }

    #[test]
    fn sample_input_part_2() {}
}
