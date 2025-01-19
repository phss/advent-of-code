use std::env;

use advent_of_code_2024::{day1, day2, day3, day4, day5, day6, day7, day8};

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
        _ => panic!("No solution yet for day {} and part {}", day, part),
    };
    solution()
}

fn main() {
    let (day, part) = parse_command_line_args();
    let result = run(day, part);
    println!("Day {} part {} answer: {}", day, part, result)
}
