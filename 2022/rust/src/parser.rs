use std::{fs, str::FromStr};

pub fn read<T: FromStr>(filename: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(FromStr::from_str)
        .collect()
}
