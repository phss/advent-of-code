use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::parser;

type Position = (usize, usize);

struct Problem {
    start: Position,
    end: Position,
    map: Vec<Vec<u32>>,
}

impl Problem {
    fn parse(lines: &Vec<String>) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);

        for y in 0..lines.len() {
            for (x, c) in lines[y].chars().enumerate() {
                if c == 'S' {
                    start = (x, y);
                } else if c == 'E' {
                    end = (x, y);
                }
            }
        }

        let map = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => 'a' as u32,
                        'E' => 'z' as u32,
                        c => c as u32,
                    })
                    .collect()
            })
            .collect();

        Self { start, end, map }
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day12.txt").unwrap();
    let problem = Problem::parse(&lines);
    fewest_steps(problem)
}

pub fn part2() -> u32 {
    let lines: Vec<String> = parser::read("data/day12.txt").unwrap();
    let problem = Problem::parse(&lines);
    fewest_steps_from_base(problem)
}

fn fewest_steps(problem: Problem) -> u32 {
    let width = problem.map[0].len();
    let height = problem.map.len();
    let mut considered: HashSet<Rc<Position>> = HashSet::new();

    let mut next_positions = VecDeque::new();
    let start = Rc::new(problem.start);
    next_positions.push_back((start.clone(), 0));
    considered.insert(Rc::clone(&start));

    loop {
        if next_positions.len() == 0 {
            return 1000000; // TODO fix this mega hardcoded break condition
        }

        let (position_rc, steps) = next_positions.pop_front().unwrap();
        let x = position_rc.0;
        let y = position_rc.1;
        if (x, y) == problem.end {
            return steps;
        }

        let candidates = vec![
            (x.checked_sub(1).unwrap_or(0), y),
            ((x + 1).min(width - 1), y),
            (x, y.checked_sub(1).unwrap_or(0)),
            (x, (y + 1).min(height - 1)),
        ];
        for (cx, cy) in candidates {
            let candidate_position = Rc::new((cx, cy));

            let valid_step = (0..=problem.map[y][x] + 1).contains(&problem.map[cy][cx]);
            let not_seen = !considered.contains(&candidate_position);

            if valid_step && not_seen {
                next_positions.push_back((candidate_position.clone(), steps + 1));
                considered.insert(Rc::clone(&candidate_position));
            }
        }
    }
}

fn fewest_steps_from_base(problem: Problem) -> u32 {
    find_starts(&problem)
        .iter()
        .map(|(x, y)| {
            fewest_steps(Problem {
                start: (*x, *y),
                end: problem.end,
                map: problem.map.clone(),
            })
        })
        .min()
        .unwrap()
}

fn find_starts(problem: &Problem) -> Vec<Position> {
    let mut starts = vec![];

    for y in 0..problem.map.len() {
        for (x, &c) in problem.map[y].iter().enumerate() {
            if c == 'a' as u32 {
                starts.push((x, y));
            }
        }
    }

    starts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let lines = vec![String::from("Sacde"), String::from("fghiE")];
        let problem = Problem::parse(&lines);
        assert_eq!(problem.start, (0, 0));
        assert_eq!(problem.end, (4, 1));
        assert_eq!(
            problem.map,
            vec![vec![97, 97, 99, 100, 101], vec![102, 103, 104, 105, 122]]
        );
    }

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];
        let problem = Problem::parse(&lines);
        assert_eq!(fewest_steps(problem), 31);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            String::from("Sabqponm"),
            String::from("abcryxxl"),
            String::from("accszExk"),
            String::from("acctuvwj"),
            String::from("abdefghi"),
        ];
        let problem = Problem::parse(&lines);
        assert_eq!(fewest_steps_from_base(problem), 29);
    }
}
