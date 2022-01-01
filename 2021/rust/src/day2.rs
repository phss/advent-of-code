use std::fs;

pub fn part1() -> u32 {
    0
}

fn parse(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    contents
        .split("\n")
        .map(|line| line.parse().expect("not a number"))
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
