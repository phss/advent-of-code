use crate::parser;

type Board = Vec<Vec<u32>>;

#[derive(Debug)]
struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

fn parse(lines: &Vec<String>) -> Input {
    let numbers: Vec<u32> = lines
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    Input {
        numbers,
        boards: vec![],
    }
}

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/temp.txt").unwrap();
    println!("{:?}", parse(&lines));
    0
}
