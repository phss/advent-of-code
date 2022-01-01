use std::fs;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

pub fn part1() -> u32 {
    let commands = parse("data/temp.txt");
    println!("{:?}", commands);
    0
}

fn parse(filename: &str) -> Vec<Command> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    contents
        .split("\n")
        .map(|line| {
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
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blah() {
        assert_eq!(1, 1);
    }
}
