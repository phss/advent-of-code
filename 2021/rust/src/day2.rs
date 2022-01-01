use crate::parser;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn part1() -> u32 {
    let commands = parser::read("data/temp.txt", to_command);
    println!("{:?}", commands);
    0
}

fn to_command(line: &str) -> Command {
    let mut splitted_line = line.split_whitespace();
    let command = match splitted_line.next() {
        Some("forward") => Command::Forward,
        Some("down") => Command::Down,
        Some("up") => Command::Up,
        _ => panic!("unrecognised command"),
    };
    let value = splitted_line
        .next()
        .map(|v| v.parse().unwrap())
        .expect("malformed value");
    command(value)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blah() {
        assert_eq!(1, 1);
    }
}
