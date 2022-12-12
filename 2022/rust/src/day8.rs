use std::collections::HashSet;

use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day8.txt").unwrap();
    let tree_heights: Vec<Vec<i32>> = to_height_map(&lines);
    trees_visible(&tree_heights)
}

pub fn part2() -> u32 {
    0
}

fn to_height_map(lines: &Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c as i32).collect())
        .collect()
}

fn trees_visible(tree_heights: &Vec<Vec<i32>>) -> u32 {
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..tree_heights.len() {
        let mut tallest_height = -1;
        for x in 0..tree_heights[y].len() {
            let height = tree_heights[y][x];
            if height > tallest_height {
                visible_trees.insert((x, y));
                tallest_height = height;
            }
        }
    }

    for y in 0..tree_heights.len() {
        let mut tallest_height = -1;
        for x in (0..tree_heights[y].len()).rev() {
            let height = tree_heights[y][x];
            if height > tallest_height {
                visible_trees.insert((x, y));
                tallest_height = height;
            }
        }
    }

    for x in 0..tree_heights[0].len() {
        let mut tallest_height = -1;
        for y in 0..tree_heights.len() {
            let height = tree_heights[y][x];
            if height > tallest_height {
                visible_trees.insert((x, y));
                tallest_height = height;
            }
        }
    }
    for x in 0..tree_heights[0].len() {
        let mut tallest_height = -1;
        for y in (0..tree_heights.len()).rev() {
            let height = tree_heights[y][x];
            if height > tallest_height {
                visible_trees.insert((x, y));
                tallest_height = height;
            }
        }
    }

    visible_trees.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let tree_heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(trees_visible(&tree_heights), 21);
    }

    #[test]
    fn sample_input_part_2() {}
}
