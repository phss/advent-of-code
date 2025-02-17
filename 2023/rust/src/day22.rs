use std::str::FromStr;

use crate::parser;

#[derive(Debug)]
struct Brick {
    from: (usize, usize, usize),
    to: (usize, usize, usize),
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('~').collect();

        let from_parts: Vec<usize> = parts[0]
            .split(',')
            .map(|x| x.parse().map_err(|_| format!("Invalid number: {}", x)))
            .collect::<Result<Vec<_>, _>>()?;
        let to_parts: Vec<usize> = parts[1]
            .split(',')
            .map(|x| x.parse().map_err(|_| format!("Invalid number: {}", x)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Brick {
            from: (from_parts[0], from_parts[1], from_parts[2]),
            to: (to_parts[0], to_parts[1], to_parts[2]),
        })
    }
}

pub fn part1() -> usize {
    let bricks: Vec<Brick> = parser::read("data/day22.txt").unwrap();
    count_desintegrate(&bricks)
}

pub fn part2() -> usize {
    0
}

fn count_desintegrate(bricks: &Vec<Brick>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "1,0,1~1,2,1",
            "0,0,2~2,0,2",
            "0,2,3~2,2,3",
            "0,0,4~0,2,4",
            "2,0,5~2,2,5",
            "0,1,6~2,1,6",
            "1,1,8~1,1,9",
        ];
        let bricks: Vec<Brick> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_desintegrate(&bricks);

        assert_eq!(result, 5);
    }

    #[test]
    fn sample_input_part_2() {}
}
