
use itertools::Itertools;

use crate::parser;

pub fn part1() -> u32 {
    let signal: Vec<String> = parser::read("data/day6.txt").unwrap();
    marker_index(signal.first().unwrap())
}

pub fn part2() -> u32 {
    let signal: Vec<String> = parser::read("data/day6.txt").unwrap();
    start_of_message(signal.first().unwrap())
}

fn marker_index(signal: &str) -> u32 {
    let potential_markers: Vec<char> = signal.chars().collect();
    let marker = potential_markers
        .windows(4)
        .enumerate()
        .find(|(_, marker)| marker.into_iter().all_unique());
    (marker.unwrap().0 + 4) as u32
}

fn start_of_message(signal: &str) -> u32 {
    let potential_markers: Vec<char> = signal.chars().collect();
    let marker = potential_markers
        .windows(14)
        .enumerate()
        .find(|(_, marker)| marker.into_iter().all_unique());
    (marker.unwrap().0 + 14) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_marker_index() {
        assert_eq!(marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(marker_index("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn sample_input_start_of_message() {
        assert_eq!(start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
