use std::str::FromStr;

use crate::parser;

enum Shape {
    Rock,
    Paper,
    Scissor,
}

struct Play(Shape, Shape);

#[derive(Debug, Clone)]
struct ParsePlayError;

impl FromStr for Play {
    type Err = ParsePlayError;

    fn from_str(s: &str) -> Result<Self, ParsePlayError> {
        let mut splitted_line = s.split_whitespace();
        let opponent = splitted_line.next().and_then(|v| match v {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissor),
            _ => None,
        });
        let response = splitted_line.next().and_then(|v| match v {
            "X" => Some(Shape::Rock),
            "Y" => Some(Shape::Paper),
            "Z" => Some(Shape::Scissor),
            _ => None,
        });
        match (opponent, response) {
            (Some(opponent), Some(response)) => Ok(Play(opponent, response)),
            _ => Err(ParsePlayError),
        }
    }
}

pub fn part1() -> u32 {
    let strategy_guide: Vec<Play> = parser::read("data/day2.txt").unwrap();
    total_score(&strategy_guide)
}

fn total_score(strategy_guide: &Vec<Play>) -> u32 {
    strategy_guide.iter().map(|play| score(play)).sum()
}

fn score(play: &Play) -> u32 {
    let selected_shaped_score = match play {
        Play(_, Shape::Rock) => 1,
        Play(_, Shape::Paper) => 2,
        Play(_, Shape::Scissor) => 3,
    };

    let outcome_score = match play {
        Play(Shape::Rock, Shape::Paper) => 6,
        Play(Shape::Paper, Shape::Scissor) => 6,
        Play(Shape::Scissor, Shape::Rock) => 6,
        Play(Shape::Rock, Shape::Rock) => 3,
        Play(Shape::Paper, Shape::Paper) => 3,
        Play(Shape::Scissor, Shape::Scissor) => 3,
        _ => 0,
    };

    selected_shaped_score + outcome_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_max_carried_calories() {
        let strategy_guide = vec![
            Play(Shape::Rock, Shape::Paper),
            Play(Shape::Paper, Shape::Rock),
            Play(Shape::Scissor, Shape::Scissor),
        ];
        assert_eq!(total_score(&strategy_guide), 15);
    }
}
