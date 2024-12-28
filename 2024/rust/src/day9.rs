use num_bigint::BigInt;

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day9.txt").unwrap();
    let mut disk_map = parse_disk_map(lines);
    println!("{}", compact_checksum(&mut disk_map));
    0
}

pub fn part2() -> u32 {
    0
}

fn parse_disk_map(lines: Vec<String>) -> Vec<u32> {
    lines
        .first()
        .unwrap()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect()
}

fn compact_checksum(disk_map: &mut Vec<u32>) -> BigInt {
    let mut checksum: BigInt = 0.into();

    let mut disk_map_idx = 0;
    let mut compact_idx = 0;
    let mut last_file_size_idx = disk_map.len() - 1;

    while disk_map_idx <= last_file_size_idx {
        let file_id = disk_map_idx / 2;
        let file_size = disk_map[disk_map_idx];
        let free_space_size = disk_map[disk_map_idx + 1];

        (0..file_size).for_each(|_| {
            checksum += file_id * compact_idx;
            compact_idx += 1;
        });

        (0..free_space_size).for_each(|_| {
            while disk_map[last_file_size_idx] == 0 {
                last_file_size_idx -= 2;
            }
            if last_file_size_idx > disk_map_idx {
                let last_file_id = last_file_size_idx / 2;
                checksum += last_file_id * compact_idx;
                compact_idx += 1;
                disk_map[last_file_size_idx] -= 1;
            }
        });

        disk_map_idx += 2;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let mut disk_map = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];

        let result = compact_checksum(&mut disk_map);

        assert_eq!(result, 1928.into())
    }

    #[test]
    fn sample_input_part_2() {}
}
