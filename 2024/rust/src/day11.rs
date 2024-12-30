use cached::proc_macro::cached;
use num_bigint::BigInt;

pub fn part1() -> u32 {
    let input = "4022724 951333 0 21633 5857 97 702 6";
    let stones = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let result = stones_count_after_blinks(&stones, 25);
    println!("Result: {}", result);
    0
}

pub fn part2() -> u32 {
    let input = "4022724 951333 0 21633 5857 97 702 6";
    let stones = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let result = stones_count_after_blinks(&stones, 75);
    println!("Result: {}", result);
    0
}

fn stones_count_after_blinks(stones: &Vec<String>, blinks: u32) -> BigInt {
    stones
        .iter()
        .map(|stone| stone_count(stone.clone(), blinks))
        .sum()
}

#[cached]
fn stone_count(stone: String, blinks: u32) -> BigInt {
    if blinks == 0 {
        return 1.into();
    }

    let stone = stone.trim_start_matches('0');

    if stone == "" {
        stone_count("1".to_string(), blinks - 1)
    } else if stone.len() % 2 == 0 {
        let (left, right) = stone.split_at(stone.len() / 2);
        stone_count(left.to_string(), blinks - 1) + stone_count(right.to_string(), blinks - 1)
    } else {
        let mut new_stone: BigInt = stone.parse().unwrap();
        let mult: BigInt = 2024.into();
        new_stone *= mult;
        stone_count(new_stone.to_string(), blinks - 1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_input_part_1() {
        let stones = vec!["125", "17"];
        let stones: Vec<String> = stones.into_iter().map(|s| s.to_string()).collect();

        let result = stones_count_after_blinks(&stones, 25);

        assert_eq!(result, 55312.into());
    }

    #[test]
    fn sample_input_part_2() {}
}
