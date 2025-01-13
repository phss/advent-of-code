use std::str::FromStr;

use crate::parser;

type Show = (usize, usize, usize);

#[derive(Debug)]
struct Game {
    id: usize,
    shows: Vec<Show>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_string());
        }

        let id_part = parts[0];
        let shows_part = parts[1];

        let id = id_part
            .strip_prefix("Game ")
            .ok_or("Invalid game id format")?
            .parse::<usize>()
            .map_err(|_| "Invalid game id")?;

        let shows: Vec<Show> = shows_part
            .split("; ")
            .map(|show| {
                let mut counts = [0; 3];
                for part in show.split(", ") {
                    let mut iter = part.split_whitespace();
                    let count = iter.next().unwrap().parse::<usize>().unwrap();
                    let color = iter.next().unwrap();
                    match color {
                        "red" => counts[0] = count,
                        "green" => counts[1] = count,
                        "blue" => counts[2] = count,
                        _ => return Err("Invalid color".to_string()),
                    }
                }
                Ok((counts[0], counts[1], counts[2]))
            })
            .collect::<Result<Vec<Show>, _>>()
            .unwrap();

        Ok(Game { id, shows })
    }
}

pub fn part1() -> u32 {
    let games: Vec<Game> = parser::read("data/day2.txt").unwrap();
    count_possible_games(&games) as u32
}

pub fn part2() -> u32 {
    0
}

fn count_possible_games(games: &Vec<Game>) -> usize {
    games
        .iter()
        .filter(|game| {
            game.shows
                .iter()
                .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
        })
        .map(|game| game.id)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games: Vec<Game> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_possible_games(&games);

        assert_eq!(result, 8)
    }

    #[test]
    fn sample_input_part_2() {}
}
