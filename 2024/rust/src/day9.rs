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
    let mut array = into_disk_array(disk_map);
    let mut front_idx = 0;
    let mut back_idx = array.len() - 1;

    loop {
        while array[front_idx] != EMPTY {
            front_idx += 1;
        }
        while array[back_idx] == EMPTY {
            back_idx -= 1;
        }
        if front_idx >= back_idx {
            break;
        }

        array.swap(front_idx, back_idx);
    }

    checksum(&array)
}

fn whole_file_compact_checksum(disk_map: &mut Vec<u32>) -> BigInt {
    let mut array = into_disk_array(disk_map);
    let (file_locs, mut empty_locs) = extract_locs(disk_map);

    for (file_size, file_idx) in file_locs.iter().rev() {
        let first_empty = empty_locs.iter().find_position(|(empty_size, empty_idx)| {
            empty_size >= file_size && empty_idx < file_idx
        });

        if let Some((loc_idx, (_, empty_idx))) = first_empty {
            (0..*file_size).for_each(|i| {
                let i = i as usize;
                array.swap(empty_idx + i, file_idx + i);
            });

            empty_locs[loc_idx].0 -= file_size;
            empty_locs[loc_idx].1 += *file_size as usize;
        }
    }

    checksum(&array)
}

fn into_disk_array(disk_map: &mut Vec<u32>) -> Vec<i32> {
    let size: u32 = disk_map.iter().sum();
    let mut array = vec![EMPTY; size as usize];
    let mut array_idx = 0;

    disk_map.push(0);
    for (idx, (file_size, free_space_size)) in disk_map.iter().tuples().enumerate() {
        let id = idx as i32;

        (0..*file_size).for_each(|_| {
            array[array_idx] = id;
            array_idx += 1;
        });
        array_idx += *free_space_size as usize;
    }

    array
}

fn extract_locs(disk_map: &mut Vec<u32>) -> (Vec<(u32, usize)>, Vec<(u32, usize)>) {
    let mut files = Vec::new();
    let mut empties = Vec::new();
    let mut array_idx = 0;

    for (file_size, free_space_size) in disk_map.iter().tuples() {
        files.push((*file_size, array_idx));
        array_idx += *file_size as usize;

        empties.push((*free_space_size, array_idx));
        array_idx += *free_space_size as usize;
    }

    (files, empties)
}

fn checksum(disk_alloc: &Vec<i32>) -> BigInt {
    let mut checksum: BigInt = 0.into();

    for (idx, id) in disk_alloc.iter().enumerate() {
        if *id >= 0 {
            checksum += *id as u32 * idx as u32;
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

        assert_eq!(result, 2858.into())
    }
}
