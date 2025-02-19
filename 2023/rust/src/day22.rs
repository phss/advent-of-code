use std::str::FromStr;

use crate::parser;

#[derive(Debug, Clone)]
struct Brick {
    from: (usize, usize, usize),
    to: (usize, usize, usize),
}

impl Brick {
    fn is_lower(&self, other: &Brick) -> bool {
        self.to.2 < other.from.2
    }

    fn overlaps(&self, other: &Brick) -> bool {
        self.from.0 <= other.to.0
            && self.to.0 >= other.from.0
            && self.from.1 <= other.to.1
            && self.to.1 >= other.from.1
    }

    fn distance(&self, other: &Brick) -> usize {
        other.from.2 - self.to.2 - 1
    }

    fn supported_by(&self, other: &Brick) -> bool {
        self.from.2 == (other.to.2 + 1)
    }
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
    let mut bricks: Vec<Brick> = parser::read("data/day22.txt").unwrap();
    count_desintegrate(&mut bricks)
}

pub fn part2() -> usize {
    0
}

fn count_desintegrate(bricks: &mut Vec<Brick>) -> usize {
    bricks.sort_by_key(|b| b.from.2);
    let mut fallen_bricks: Vec<Brick> = Vec::new();

    for brick in bricks.iter_mut() {
        let fall_height = fallen_bricks
            .iter()
            .filter(|b| b.is_lower(brick) && b.overlaps(brick))
            .map(|b| b.distance(brick))
            .min()
            .unwrap_or(0);

        brick.from.2 -= fall_height;
        brick.to.2 -= fall_height;
        fallen_bricks.push(brick.clone());
    }

    let mut count = 0;
    for brick in fallen_bricks.iter() {
        let supported_bricks: Vec<&Brick> = fallen_bricks
            .iter()
            .filter(|b| b.supported_by(brick) && b.overlaps(brick))
            .collect();

        if supported_bricks.iter().all(|supported| {
            fallen_bricks
                .iter()
                .filter(|b| supported.supported_by(b) && supported.overlaps(b))
                .count()
                > 1
        }) {
            count += 1;
        }
    }

    count
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
        let mut bricks: Vec<Brick> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_desintegrate(&mut bricks);

        assert_eq!(result, 5);
    }

    #[test]
    fn sample_input_part_2() {}
}
