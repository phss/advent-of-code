use crate::parser;

trait CharAtTrait {
    fn char_at(&self, x: usize, y: usize) -> Option<char>;
}

impl CharAtTrait for Vec<String> {
    fn char_at(&self, x: usize, y: usize) -> Option<char> {
        self.get(y).and_then(|line| line.chars().nth(x))
    }
}

pub fn part1() -> u32 {
    let memory: Vec<String> = parser::read("data/day4.txt").unwrap();
    xmas_count(&memory)
}

pub fn part2() -> u32 {
    let memory: Vec<String> = parser::read("data/day4.txt").unwrap();
    x_mas_count(&memory)
}

fn xmas_count(word_search: &Vec<String>) -> u32 {
    let width = word_search[0].len();
    let height = word_search.len();
    let directions = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
        (0, -1),
        (-1, 0),
    ];
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            if Some('X') == word_search.char_at(x, y) {
                for direction in directions.iter() {
                    if is_word_in_direction(word_search, "XMAS", (x, y), *direction) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn x_mas_count(word_search: &Vec<String>) -> u32 {
    let width = word_search[0].len();
    let height = word_search.len();
    let mut count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if Some('A') == word_search.char_at(x, y) {
                let top_left_down_mas =
                    is_word_in_direction(word_search, "MAS", (x - 1, y - 1), (1, 1));
                let top_right_down_mas =
                    is_word_in_direction(word_search, "MAS", (x + 1, y - 1), (-1, 1));
                let top_left_down_sam =
                    is_word_in_direction(word_search, "SAM", (x - 1, y - 1), (1, 1));
                let top_right_down_sam =
                    is_word_in_direction(word_search, "SAM", (x + 1, y - 1), (-1, 1));

                if (top_left_down_mas || top_left_down_sam)
                    && (top_right_down_mas || top_right_down_sam)
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_word_in_direction(
    word_search: &Vec<String>,
    word: &str,
    origin: (usize, usize),
    direction: (i32, i32),
) -> bool {
    let mut x = origin.0 as i32;
    let mut y = origin.1 as i32;
    let (x_inc, y_inc) = direction;

    for c in word.chars() {
        if x < 0 || y < 0 || Some(c) != word_search.char_at(x as usize, y as usize) {
            return false;
        }
        x += x_inc;
        y += y_inc;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let word_search = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let word_search: Vec<String> = word_search.into_iter().map(|s| s.to_string()).collect();

        let result = xmas_count(&word_search);

        assert_eq!(result, 18)
    }

    #[test]
    fn sample_input_part_2() {
        let word_search = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let word_search: Vec<String> = word_search.into_iter().map(|s| s.to_string()).collect();

        let result = x_mas_count(&word_search);

        assert_eq!(result, 9)
    }
}
