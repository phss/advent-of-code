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
        let (sx1, sy1, _) = self.from;
        let (sx2, sy2, _) = self.to;
        let (ox1, oy1, _) = other.from;
        let (ox2, oy2, _) = other.to;

        sx1 <= ox2 && sx2 >= ox1 && sy1 <= oy2 && sy2 >= oy1
    }

    fn distance(&self, other: &Brick) -> usize {
        other.from.2 - self.to.2 - 1
    }

    fn above(&self, other: &Brick) -> bool {
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
            .unwrap_or(brick.from.2 - 1);

        brick.from.2 -= fall_height;
        brick.to.2 -= fall_height;
        fallen_bricks.push(brick.clone());
    }

    fallen_bricks.sort_by_key(|b| b.from.2);

    let mut count = 0;
    for brick in fallen_bricks.iter() {
        let supported_bricks: Vec<&Brick> = fallen_bricks
            .iter()
            .filter(|b| b.above(brick) && b.overlaps(brick))
            .collect();

        if supported_bricks.iter().all(|supported| {
            fallen_bricks
                .iter()
                .filter(|b| supported.above(b) && supported.overlaps(b))
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
    fn sample_input_part_1_another() {
        let lines = vec!["5,1,1~1,1,1", "1,5,2~1,1,2"];
        let mut bricks: Vec<Brick> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_desintegrate(&mut bricks);

        assert_eq!(result, 2);
    }

    #[test]
    fn sample_input_part_2() {}

    #[test]
    fn overlaps() {
        let a = Brick {
            from: (0, 5, 158),
            to: (3, 5, 158),
        };

        let b = Brick {
            from: (1, 8, 158),
            to: (3, 8, 158),
        };

        assert_eq!(a.overlaps(&b), false);
    }
}
