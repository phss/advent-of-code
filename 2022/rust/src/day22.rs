use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day22.txt").unwrap();
    let (map, instructions) = parse(lines);
    final_password(&map, &instructions)
}

pub fn part2() -> usize {
    0
}

fn final_password(map: &Vec<Vec<char>>, instructions: &Vec<(usize, char)>) -> usize {
    0
}

fn parse(lines: Vec<String>) -> (Vec<Vec<char>>, Vec<(usize, char)>) {
    let mut raw = lines.split(|line| line.is_empty());

    let map = raw
        .next()
        .unwrap()
        .iter()
        .map(|s| s.chars().collect())
        .collect();

    let direction = |c| c == 'R' || c == 'L';
    let instruction_raw = raw.next().unwrap().first().unwrap();
    let instructions = instruction_raw
        .split_inclusive(direction)
        .map(|elem| {
            let (num, dir) = if elem.ends_with(direction) {
                elem.split_at(elem.len() - 1)
            } else {
                (elem, "")
            };
            (
                num.parse::<usize>().unwrap(),
                dir.chars().next().unwrap_or(' '),
            )
        })
        .collect();

    (map, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let input = vec![
            "        ...#    ",
            "        .#..    ",
            "        #...    ",
            "        ....    ",
            "...#.......#    ",
            "........#...    ",
            "..#....#....    ",
            "..........#.    ",
            "        ...#....",
            "        .....#..",
            "        .#......",
            "        ......#.",
            "",
            "10R5L5R10L4R5L5",
        ];
        let lines: Vec<String> = input.iter().map(|s| s.parse().unwrap()).collect();
        let (map, instructions) = parse(lines);

        let result = final_password(&map, &instructions);

        assert_eq!(result, 6032);
    }

    #[test]
    fn sample_input_part_2() {}
}
