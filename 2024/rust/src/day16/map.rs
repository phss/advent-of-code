pub fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut raw = lines.split(|line| line.is_empty());

    raw.next()
        .unwrap()
        .iter()
        .map(|s| s.chars().collect())
        .collect()
}

pub fn position(map: &Vec<Vec<char>>, char: char) -> (usize, usize) {
    let mut position = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == char) {
            position = (x, y);
            break;
        }
    }
    position
}

// pub fn print(map: &Vec<Vec<char>>) {
//     for row in map {
//         for c in row {
//             print!("{}", c);
//         }
//         println!("");
//     }
// }
