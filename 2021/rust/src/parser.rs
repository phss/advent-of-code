use std::{fmt::Debug, fs, str::FromStr};

pub fn read<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(filename).expect("unable to read file");
    contents
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect()
}
