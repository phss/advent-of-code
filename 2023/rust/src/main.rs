use std::env;

use advent_of_code_2024::{
    day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day3, day4,
    day5, day6, day7, day8, day9,
};

fn parse_command_line_args() -> (u32, u32) {
    let args: Vec<String> = env::args().collect();
    let parse_numeric = |arg: &String| arg.parse::<u32>();

    match args.len() {
        3 => (
            parse_numeric(&args[1]).expect("invalid day"),
            parse_numeric(&args[2]).expect("invalid part"),
        ),
        _ => panic!("wrong number of args"),
    }
}

fn run(day: u32, part: u32) -> usize {
    let solution = match (day, part) {
        (1, 1) => day1::part1,
        (1, 2) => day1::part2,
        (2, 1) => day2::part1,
        (2, 2) => day2::part2,
        (3, 1) => day3::part1,
        (3, 2) => day3::part2,
        (4, 1) => day4::part1,
        (4, 2) => day4::part2,
        (5, 1) => day5::part1,
        (5, 2) => day5::part2,
        (6, 1) => day6::part1,
        (6, 2) => day6::part2,
        (7, 1) => day7::part1,
        (7, 2) => day7::part2,
        (8, 1) => day8::part1,
        (8, 2) => day8::part2,
        (9, 1) => day9::part1,
        (9, 2) => day9::part2,
        (10, 1) => day10::part1,
        (10, 2) => day10::part2,
        (11, 1) => day11::part1,
        (11, 2) => day11::part2,
        (12, 1) => day12::part1,
        (12, 2) => day12::part2,
        (13, 1) => day13::part1,
        (13, 2) => day13::part2,
        (14, 1) => day14::part1,
        (14, 2) => day14::part2,
        (15, 1) => day15::part1,
        (15, 2) => day15::part2,
        (16, 1) => day16::part1,
        (16, 2) => day16::part2,
        (17, 1) => day17::part1,
        (17, 2) => day17::part2,
        (18, 1) => day18::part1,
        (18, 2) => day18::part2,
        (19, 1) => day19::part1,
        (19, 2) => day19::part2,
        _ => panic!("No solution yet for day {} and part {}", day, part),
    };
    solution()
}

fn main() {
    let (day, part) = parse_command_line_args();
    let result = run(day, part);
    println!("Day {} part {} answer: {}", day, part, result)
}
