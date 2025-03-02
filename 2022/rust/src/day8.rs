use std::iter::repeat;

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day8.txt").unwrap();
    let tree_heights: Vec<Vec<usize>> = to_height_map(&lines);
    trees_visible(&tree_heights)
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day8.txt").unwrap();
    let tree_heights: Vec<Vec<usize>> = to_height_map(&lines);
    highest_scenic_score(&tree_heights)
}

fn to_height_map(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|line| line.chars().map(|c| c as usize).collect())
        .collect()
}

fn trees_visible(tree_heights: &Vec<Vec<usize>>) -> usize {
    let width = tree_heights[0].len();
    let height = tree_heights.len();
    let mut visible_trees = 0;

    for y in 0..height {
        for x in 0..width {
            let tree_height = tree_heights[y][x];
            if is_highest_tree_in_dir(tree_heights, (0..x).zip(repeat(y)), tree_height)
                || is_highest_tree_in_dir(tree_heights, (x + 1..width).zip(repeat(y)), tree_height)
                || is_highest_tree_in_dir(tree_heights, repeat(x).zip(0..y), tree_height)
                || is_highest_tree_in_dir(tree_heights, repeat(x).zip(y + 1..height), tree_height)
            {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn is_highest_tree_in_dir<I>(
    tree_heights: &Vec<Vec<usize>>,
    tree_coords_in_direction: I,
    current_height: usize,
) -> bool
where
    I: Iterator<Item = (usize, usize)>,
{
    tree_coords_in_direction
        .map(|(x, y)| tree_heights[y][x])
        .all(|tree_height| tree_height < current_height)
}

fn highest_scenic_score(tree_heights: &Vec<Vec<usize>>) -> usize {
    let width = tree_heights[0].len();
    let height = tree_heights.len();
    let mut max_scenic_score = 0;

    for y in 0..height {
        for x in 0..width {
            let tree_height = tree_heights[y][x];
            let left = scenic_score_in_dir(tree_heights, (0..x).rev().zip(repeat(y)), tree_height, x);
            let right = scenic_score_in_dir(tree_heights, (x + 1..width).zip(repeat(y)), tree_height, width-x-1);
            let up = scenic_score_in_dir(tree_heights, repeat(x).zip((0..y).rev()), tree_height, y);
            let down = scenic_score_in_dir(tree_heights, repeat(x).zip(y + 1..height), tree_height, height-y-1);

            let scenic_score = left * right * up * down;

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score as usize
}

fn scenic_score_in_dir<I>(
    tree_heights: &Vec<Vec<usize>>,
    tree_coords_in_direction: I,
    current_height: usize,
    default: usize
) -> usize
where
    I: Iterator<Item = (usize, usize)>,
{
    let first_blocking_tree = tree_coords_in_direction
        .enumerate()
        .map(|(i, (x, y))| (i, tree_heights[y][x]))
        .find(|(_, tree_height)| tree_height >= &current_height)
        .map(|(i, _)| i + 1);

    match first_blocking_tree {
        Some(i) => i,
        None => default,
    }
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
    fn sample_input_part_2() {
        let tree_heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(highest_scenic_score(&tree_heights), 8);
    }
}
