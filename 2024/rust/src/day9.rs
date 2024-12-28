use std::{collections::VecDeque, ops::Neg};

use itertools::Itertools;
use num_bigint::BigInt;

use crate::parser;

const EMPTY: i32 = -1;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day9.txt").unwrap();
    let mut disk_map = parse_disk_map(lines);
    println!("{}", compact_checksum(&mut disk_map));
    0
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day9.txt").unwrap();
    let mut disk_map = parse_disk_map(lines);
    println!("{}", whole_file_compact_checksum(&mut disk_map));
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
    let mut disk_alloc = into_disk_alloc(disk_map);
    let mut front_idx = 0;
    let mut back_idx = disk_alloc.len() - 1;

    loop {
        while disk_alloc[front_idx] != EMPTY {
            front_idx += 1;
        }
        while disk_alloc[back_idx] == EMPTY {
            back_idx -= 1;
        }
        if front_idx >= back_idx {
            break;
        }

        disk_alloc.swap(front_idx, back_idx);
    }

    checksum(&disk_alloc)
}

fn whole_file_compact_checksum(disk_map: &mut Vec<u32>) -> BigInt {
    // let mut disk_alloc = VecDeque::from(into_disk_alloc(disk_map));
    // let mut compact_alloc = Vec::new();

    // let mut front_node = disk_alloc.pop_front();
    // let mut back_node = disk_alloc.pop_back();

    // checksum(&disk_alloc)
    0.into()
}

fn into_disk_alloc(disk_map: &mut Vec<u32>) -> Vec<i32> {
    let size: u32 = disk_map.iter().sum();
    let mut disk_alloc = vec![EMPTY; size as usize];
    let mut alloc_id = 0;

    disk_map.push(0);
    for (idx, (file_size, free_space_size)) in disk_map.iter().tuples().enumerate() {
        let id = idx as i32;

        (0..*file_size).for_each(|_| {
            disk_alloc[alloc_id] = id;
            alloc_id += 1;
        });
        alloc_id += *free_space_size as usize;
    }

    disk_alloc
}

fn checksum(disk_alloc: &Vec<i32>) -> BigInt {
    let mut checksum: BigInt = 0.into();

    for (idx, id) in disk_alloc.iter().enumerate() {
        if *id >= 0 {
            checksum += *id as u32 * idx as u32
        }
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
    fn sample_input_part_2() {
        let mut disk_map = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];

        let result = whole_file_compact_checksum(&mut disk_map);

        // assert_eq!(result, 2858.into())
    }
}
