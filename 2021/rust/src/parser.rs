use std::fs;

pub fn read<T>(filename: &str, parse_line: fn(&str) -> T) -> Vec<T> {
    let contents = fs::read_to_string(filename).expect("unable to read file");
    contents.split("\n").map(parse_line).collect()
}

pub fn to_number(line: &str) -> u32 {
    line.parse().expect("not a number")
}
