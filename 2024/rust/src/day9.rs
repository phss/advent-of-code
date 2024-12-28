use std::{collections::VecDeque, ops::Neg};

use itertools::Itertools;
use num_bigint::BigInt;

use crate::parser;

#[derive(Debug, Clone, Copy)]
struct DiskAlloc {
    id: u32,
    size: u32,
    free_space: u32,
}

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
    let mut disk_alloc = VecDeque::from(into_disk_alloc(disk_map));
    let mut compact_alloc = Vec::new();

    let mut front_node = disk_alloc.pop_front();
    let mut back_node = disk_alloc.pop_back();

    loop {
        while front_node.is_some_and(|node| node.free_space == 0) {
            compact_alloc.push(front_node.unwrap());
            front_node = disk_alloc.pop_front();
        }
        while back_node.is_some_and(|node| node.size == 0) {
            back_node = disk_alloc.pop_back();
        }
        if front_node.is_none() || back_node.is_none() {
            break;
        }

        let mut free_space_node = front_node.unwrap();
        let mut to_compact_node = back_node.unwrap();

        let space_after_compact = free_space_node.free_space as i32 - to_compact_node.size as i32;

        let compacted_node = DiskAlloc {
            id: to_compact_node.id,
            size: to_compact_node.size.min(free_space_node.free_space),
            free_space: space_after_compact.max(0) as u32,
        };

        free_space_node.free_space = 0;
        compact_alloc.push(free_space_node);

        to_compact_node.size = space_after_compact.neg().max(0) as u32;

        front_node = Some(compacted_node);
        back_node = Some(to_compact_node);
    }

    if let Some(node) = front_node {
        compact_alloc.push(node);
    }
    if let Some(node) = back_node {
        compact_alloc.push(node);
    }

    checksum(&compact_alloc)
}

fn whole_file_compact_checksum(disk_map: &mut Vec<u32>) -> BigInt {
    let disk_alloc = into_disk_alloc(disk_map);

    checksum(&disk_alloc)
}

fn into_disk_alloc(disk_map: &mut Vec<u32>) -> Vec<DiskAlloc> {
    let mut disk_alloc = Vec::new();
    disk_map.push(0);

    for (idx, (file_size, free_space_size)) in disk_map.iter().tuples().enumerate() {
        disk_alloc.push(DiskAlloc {
            id: idx as u32,
            size: *file_size,
            free_space: *free_space_size,
        });
    }

    disk_alloc
}

fn checksum(disk_alloc: &Vec<DiskAlloc>) -> BigInt {
    let mut checksum: BigInt = 0.into();
    let mut checksum_idx = 0;

    for disk in disk_alloc {
        (0..disk.size).for_each(|_| {
            checksum += disk.id * checksum_idx;
            checksum_idx += 1;
        });
        checksum_idx += disk.free_space;
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
