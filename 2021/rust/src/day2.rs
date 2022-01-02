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
        let command = match splitted_line.next() {
            Some("forward") => Command::Forward,
            Some("down") => Command::Down,
            Some("up") => Command::Up,
            _ => return Err(ParseCommandError),
        };
        let value = splitted_line
            .next()
            .map(|v| v.parse().unwrap())
            .expect("malformed value");
        Ok(command(value))
    }
}

pub fn part1() -> u32 {
    let commands: Vec<Command> = parser::read("data/temp.txt").unwrap();
    println!("{:?}", commands);
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn blah() {
//         assert_eq!(1, 1);
//     }
// }
