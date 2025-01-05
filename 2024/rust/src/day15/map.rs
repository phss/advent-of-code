pub fn parse(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<char>) {
    let mut raw = lines.split(|line| line.is_empty());

    let map: Vec<Vec<char>> = raw
        .next()
        .unwrap()
        .iter()
        .map(|s| s.chars().collect())
        .collect();
    let moves = raw.next().unwrap().iter().flat_map(|s| s.chars()).collect();

    (map, moves)
}

pub fn widen(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    map.iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| match c {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '@' => vec!['@', '.'],
                    _ => vec![],
                })
                .collect()
        })
        .collect()
}

pub fn robot_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut position = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '@') {
            position = (x, y);
            break;
        }
    }
    position
}

pub fn print(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

pub fn score(map: &Vec<Vec<char>>) -> u32 {
    let box_chars = ['O', '['];
    let mut total = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if box_chars.contains(c) {
                total += x + (100 * y);
            }
        }
    }

    total as u32
}
