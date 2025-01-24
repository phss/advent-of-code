pub fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut raw = lines.split(|line| line.is_empty());

    raw.next()
        .unwrap()
        .iter()
        .map(|s| s.chars().collect())
        .collect()
}

pub fn positions(map: &Vec<Vec<char>>, char: char) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == char {
                positions.push((x, y));
            }
        }
    }

    positions
}
