use std::str::FromStr;

use crate::parser;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Clone)]
struct ParseCommandError;

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, ParseCommandError> {
        let mut splitted_line = s.split_whitespace();
        let command = splitted_line
            .next()
            .and_then(|v| -> Option<fn(u32) -> Command> {
                match v {
                    "forward" => Some(Command::Forward),
                    "down" => Some(Command::Down),
                    "up" => Some(Command::Up),
                    _ => None,
                }
            });
        let value: Option<u32> = splitted_line.next().and_then(|v| v.parse().ok());
        match (command, value) {
            (Some(command), Some(value)) => Ok(command(value)),
            _ => Err(ParseCommandError),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Location {
    horizontal_position: u32,
    depth: u32,
}

pub fn part1() -> u32 {
    let commands: Vec<Command> = parser::read("data/day2.txt").unwrap();
    let location = estimate_location(commands);
    location.horizontal_position * location.depth
}

pub fn part2() -> u32 {
    let commands: Vec<Command> = parser::read("data/day2.txt").unwrap();
    let location = estimate_location_with_aim(commands);
    location.horizontal_position * location.depth
}

fn estimate_location(commands: Vec<Command>) -> Location {
    let mut location = Location {
        horizontal_position: 0,
        depth: 0,
    };

    for command in commands {
        match command {
            Command::Forward(value) => location.horizontal_position += value,
            Command::Down(value) => location.depth += value,
            Command::Up(value) => location.depth -= value,
        }
    }

    location
}

fn estimate_location_with_aim(commands: Vec<Command>) -> Location {
    let mut aim = 0;
    let mut location = Location {
        horizontal_position: 0,
        depth: 0,
    };

    for command in commands {
        match command {
            Command::Forward(value) => {
                location.horizontal_position += value;
                location.depth += aim * value;
            }
            Command::Down(value) => aim += value,
            Command::Up(value) => aim -= value,
        }
    }

    location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_estimated_position() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(
            estimate_location(commands),
            Location {
                horizontal_position: 15,
                depth: 10
            }
        );
    }

    #[test]
    fn sample_input_estimated_position_with_aim() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(
            estimate_location_with_aim(commands),
            Location {
                horizontal_position: 15,
                depth: 60
            }
        );
    }
}
