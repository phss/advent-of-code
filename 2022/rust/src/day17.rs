use std::ops::Neg;

use crate::parser;

#[derive(Debug, Clone, PartialEq)]
enum Move {
    Left,
    Right,
}

fn parse(line: &str) -> Vec<Move> {
    line.chars()
        .map(|c| match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => {
                panic!()
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Shape {
    positions: Vec<(i32, i32)>,
}

impl Shape {
    fn from(positions: Vec<(i32, i32)>) -> Self {
        Self { positions }
    }

    fn move_by(&mut self, (dx, dy): (i32, i32)) {
        self.positions.iter_mut().for_each(|position| {
            position.0 += dx;
            position.1 += dy;
        });
    }

    fn collides(&self, board: &Vec<Vec<bool>>) -> bool {
        self.positions.iter().any(|(x, y)| {
            let out_of_bounds = *x < 0 || *x > 6 || *y < 0;
            let occupied = board
                .get(*y as usize)
                .and_then(|row| row.get(*x as usize))
                .map(|v| *v)
                .unwrap_or(false);
            out_of_bounds || occupied
        })
    }

    fn transfer_to(&self, board: &mut Vec<Vec<bool>>) {
        for (x, y) in &self.positions {
            if *y as usize == board.len() {
                board.push(vec![false; 7]);
            }
            board[*y as usize][*x as usize] = true;
        }
    }

    fn digits_from(&self, board: &Vec<Vec<bool>>) -> Vec<usize> {
        let ys: Vec<i32> = self.positions.iter().map(|p| p.1).collect();
        let ymin = ys.iter().min().unwrap().clone();
        let ymax = ys.iter().max().unwrap().clone();

        (ymin..=ymax)
            .map(|y| {
                let row = board.get(y as usize).unwrap();
                let mut height = 0;
                for (i, &cell) in row.iter().enumerate() {
                    if cell {
                        height |= 1 << i;
                    }
                }
                height
            })
            .collect()
    }
}

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day17.txt").unwrap();
    let moves = parse(&lines[0]);
    let board = board_after_rocks(&moves, 2022);
    board.len()
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day17.txt").unwrap();
    let moves = parse(&lines[0]);
    simulated_height(&moves, 1000000000000)
}

fn board_after_rocks(moves: &Vec<Move>, limit: usize) -> Vec<Vec<bool>> {
    let mut moves = moves.iter().cycle();
    let mut board: Vec<Vec<bool>> = vec![];

    let shapes = vec![
        Shape::from(vec![(2, 0), (3, 0), (4, 0), (5, 0)]),
        Shape::from(vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)]),
        Shape::from(vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]),
        Shape::from(vec![(2, 0), (2, 1), (2, 2), (2, 3)]),
        Shape::from(vec![(2, 0), (3, 0), (2, 1), (3, 1)]),
    ];
    let mut shapes = shapes.iter().cycle();
    let mut count = 0;

    while count < limit {
        let mut shape = shapes.next().unwrap().clone();
        shape.move_by((0, (board.len() + 3) as i32));

        loop {
            let vertical_move = match moves.next().unwrap() {
                Move::Left => (-1, 0),
                Move::Right => (1, 0),
            };
            shape.move_by(vertical_move);
            if shape.collides(&board) {
                shape.move_by((vertical_move.0.neg(), 0));
            }

            shape.move_by((0, -1));
            if shape.collides(&board) {
                shape.move_by((0, 1));
                shape.transfer_to(&mut board);
                break;
            }
        }

        count += 1;
    }

    board
}

fn simulated_height(moves: &Vec<Move>, limit: usize) -> usize {
    let (cycle_heights, start, pre_cycle_height) = find_cycle(moves);
    let remaining_limit = limit - start;

    let cycle_height = cycle_heights.last().unwrap();
    let total_cycles = remaining_limit / cycle_heights.len();
    let cycle_i = remaining_limit & cycle_heights.len();
    let remaining_cycle_height = cycle_heights.get(cycle_i).unwrap();

    pre_cycle_height + (cycle_height * total_cycles) + remaining_cycle_height
}

fn find_cycle(moves: &Vec<Move>) -> (Vec<usize>, usize, usize) {
    let mut moves = moves.iter().enumerate().cycle();
    let mut board: Vec<Vec<bool>> = vec![];

    let shapes = vec![
        Shape::from(vec![(2, 0), (3, 0), (4, 0), (5, 0)]),
        Shape::from(vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)]),
        Shape::from(vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]),
        Shape::from(vec![(2, 0), (2, 1), (2, 2), (2, 3)]),
        Shape::from(vec![(2, 0), (3, 0), (2, 1), (3, 1)]),
    ];
    let mut shapes = shapes.iter().cycle();

    let mut entries = vec![];
    let mut cycle_start = 0;

    while cycle_start == 0 {
        let mut shape = shapes.next().unwrap().clone();
        shape.move_by((0, (board.len() + 3) as i32));

        loop {
            let (move_index, next_move) = moves.next().unwrap();
            let vertical_move = match next_move {
                Move::Left => (-1, 0),
                Move::Right => (1, 0),
            };
            shape.move_by(vertical_move);
            if shape.collides(&board) {
                shape.move_by((vertical_move.0.neg(), 0));
            }

            shape.move_by((0, -1));
            if shape.collides(&board) {
                shape.move_by((0, 1));
                shape.transfer_to(&mut board);

                let height = board.len();
                let digits = shape.digits_from(&board);
                // println!("{}: {} - {:?}", height, move_index, digits);

                let first_match_index =
                    entries
                        .iter()
                        .position(|(_, prev_move_index, prev_digits)| {
                            *prev_move_index == move_index && *prev_digits == digits
                        });

                if let Some(index) = first_match_index {
                    cycle_start = index;
                } else {
                    entries.push((height, move_index, digits));
                }

                break;
            }
        }
    }

    let pre_cycle_height = entries.get(cycle_start - 1).unwrap().0;
    let heights: Vec<usize> = (cycle_start..entries.len())
        .map(|i| {
            let entry = entries.get(i).unwrap();
            entry.0 - pre_cycle_height
        })
        .collect();

    (heights, cycle_start + 1, pre_cycle_height)
}

#[allow(dead_code)]
fn print(board: &Vec<Vec<bool>>) {
    for y in (0..board.len()).rev() {
        let line: String = board[y]
            .iter()
            .map(|p| match p {
                true => '#',
                false => '.',
            })
            .collect();
        println!("{}", line);
    }
}

#[allow(dead_code)]
fn print_as_digits(board: &Vec<Vec<bool>>) {
    let mut digits: Vec<usize> = vec![];
    for row in board {
        let mut height = 0;
        for (i, &cell) in row.iter().enumerate() {
            if cell {
                height |= 1 << i;
            }
        }
        digits.push(height);
    }

    for d in digits {
        println!("{d}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let line = "<<><>";
        assert_eq!(
            parse(line),
            vec![Move::Left, Move::Left, Move::Right, Move::Left, Move::Right]
        );
    }

    #[test]
    fn sample_input_part_1() {
        let moves = parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let board = board_after_rocks(&moves, 2022);
        assert_eq!(board.len(), 3068);
    }

    #[test]
    fn sample_input_part_2() {
        let moves = parse(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(simulated_height(&moves, 1000000000000), 1514285714288);
    }
}
