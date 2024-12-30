use num_bigint::BigInt;

pub fn part1() -> u32 {
    let input = "4022724 951333 0 21633 5857 97 702 6";
    let stones = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    stones_count_after_blinks(&stones, 25)
}

pub fn part2() -> u32 {
    0
}

fn stones_count_after_blinks(stones: &Vec<BigInt>, blinks: u32) -> u32 {
    if blinks == 0 {
        return stones.len() as u32;
    }

    let mut blinked_stones = Vec::new();

    for stone in stones {
        let stone_str = stone.to_string();

        if *stone == 0.into() {
            blinked_stones.push(1.into());
        } else if stone_str.len() % 2 == 0 {
            let (left, right) = stone_str.split_at(stone_str.len() / 2);
            blinked_stones.push(left.parse().unwrap());
            blinked_stones.push(right.parse().unwrap());
        } else {
            blinked_stones.push(stone * 2024);
        }
    }

    stones_count_after_blinks(&blinked_stones, blinks - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sample_input_part_1() {
        let stones: Vec<BigInt> = vec![125.into(), 17.into()];

        let result = stones_count_after_blinks(&stones, 25);

        assert_eq!(result, 55312);
    }

    #[test]
    fn sample_input_part_2() {}
}
