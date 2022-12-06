use std::str::FromStr;

use crate::parser;

enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

struct PlayResponse(Shape, Shape);

#[derive(Debug, Clone)]
struct ParsePlayError;

impl FromStr for PlayResponse {
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
            (Some(opponent), Some(response)) => Ok(PlayResponse(opponent, response)),
            _ => Err(ParsePlayError),
        }
    }
}

struct PlayOutcome(Shape, Outcome);

#[derive(Debug, Clone)]
struct ParsePlayOutcomeError;

impl FromStr for PlayOutcome {
    type Err = ParsePlayOutcomeError;

    fn from_str(s: &str) -> Result<Self, ParsePlayOutcomeError> {
        let mut splitted_line = s.split_whitespace();
        let opponent = splitted_line.next().and_then(|v| match v {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissor),
            _ => None,
        });
        let outcome = splitted_line.next().and_then(|v| match v {
            "X" => Some(Outcome::Lose),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        });
        match (opponent, outcome) {
            (Some(opponent), Some(outcome)) => Ok(PlayOutcome(opponent, outcome)),
            _ => Err(ParsePlayOutcomeError),
        }
    }
}

pub fn part1() -> u32 {
    let strategy_guide: Vec<PlayResponse> = parser::read("data/day2.txt").unwrap();
    total_score(&strategy_guide)
}

fn total_score(strategy_guide: &Vec<PlayResponse>) -> u32 {
    strategy_guide.iter().map(|play| score(play)).sum()
}

fn score(play: &PlayResponse) -> u32 {
    let selected_shaped_score = match play {
        PlayResponse(_, Shape::Rock) => 1,
        PlayResponse(_, Shape::Paper) => 2,
        PlayResponse(_, Shape::Scissor) => 3,
    };

    let outcome_score = match play {
        PlayResponse(Shape::Rock, Shape::Paper) => 6,
        PlayResponse(Shape::Paper, Shape::Scissor) => 6,
        PlayResponse(Shape::Scissor, Shape::Rock) => 6,
        PlayResponse(Shape::Rock, Shape::Rock) => 3,
        PlayResponse(Shape::Paper, Shape::Paper) => 3,
        PlayResponse(Shape::Scissor, Shape::Scissor) => 3,
        _ => 0,
    };

    selected_shaped_score + outcome_score
}

pub fn part2() -> u32 {
    let strategy_guide: Vec<PlayOutcome> = parser::read("data/day2.txt").unwrap();
    total_score_from_outcomes(&strategy_guide)
}

fn total_score_from_outcomes(strategy_guide: &Vec<PlayOutcome>) -> u32 {
    strategy_guide
        .iter()
        .map(|play| score_from_outcome(play))
        .sum()
}

fn score_from_outcome(play: &PlayOutcome) -> u32 {
    let outcome_score = match play {
        PlayOutcome(_, Outcome::Lose) => 0,
        PlayOutcome(_, Outcome::Draw) => 3,
        PlayOutcome(_, Outcome::Win) => 6,
    };

    let shape_score = match play {
        PlayOutcome(Shape::Rock, Outcome::Lose) => 3,
        PlayOutcome(Shape::Rock, Outcome::Draw) => 1,
        PlayOutcome(Shape::Rock, Outcome::Win) => 2,
        PlayOutcome(Shape::Paper, Outcome::Lose) => 1,
        PlayOutcome(Shape::Paper, Outcome::Draw) => 2,
        PlayOutcome(Shape::Paper, Outcome::Win) => 3,
        PlayOutcome(Shape::Scissor, Outcome::Lose) => 2,
        PlayOutcome(Shape::Scissor, Outcome::Draw) => 3,
        PlayOutcome(Shape::Scissor, Outcome::Win) => 1,
    };

    outcome_score + shape_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_total_score() {
        let strategy_guide = vec![
            PlayResponse(Shape::Rock, Shape::Paper),
            PlayResponse(Shape::Paper, Shape::Rock),
            PlayResponse(Shape::Scissor, Shape::Scissor),
        ];
        assert_eq!(total_score(&strategy_guide), 15);
    }

    #[test]
    fn sample_input_total_score_from_outcomes() {
        let strategy_guide = vec![
            PlayOutcome(Shape::Rock, Outcome::Draw),
            PlayOutcome(Shape::Paper, Outcome::Lose),
            PlayOutcome(Shape::Scissor, Outcome::Win),
        ];
        assert_eq!(total_score_from_outcomes(&strategy_guide), 12);
    }
}
