use std::{fmt::Debug, fs, io::Error, str::FromStr};

pub fn read<T>(filename: &str) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    fs::read_to_string(filename).map(|contents| {
        contents
            .split("\n")
            .map(|line| line.parse().unwrap())
            .collect()
    })
}
