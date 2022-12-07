use std::collections::HashMap;

use crate::parser;


#[derive(Clone)]
struct DirSizes<'a> {
    name: &'a str,
    size: u32,
}

pub fn part1() -> u32 {
    let command_results: Vec<String> = parser::read("data/day7.txt").unwrap();
    sub_10000_dir_sizes(&command_results)
}

pub fn part2() -> u32 {
    0
}

fn sub_10000_dir_sizes(command_results: &Vec<String>) -> u32 {
    let dirs: Vec<DirSizes> = parse_to_dir_sizes(command_results);
    dirs.into_iter()
        .filter(|dir| dir.size <= 100000)
        .map(|dir| dir.size)
        .sum()
}

fn parse_to_dir_sizes<'a>(command_results: &'a Vec<String>) -> Vec<DirSizes<'a>> {
    let mut dirs: HashMap<&str, DirSizes> = HashMap::new();
    let mut current_branch: Vec<&str> = vec![];

    for line in command_results {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        match parts[..] {
            ["$", "ls"] | ["dir", _] => {}
            ["$", "cd", ".."] => {
                current_branch.pop();
            }
            ["$", "cd", name] => {
                dirs.insert(name, DirSizes { name, size: 0 });
                current_branch.push(name);
            },
            [size, _] => {
                let size: u32 = size.parse().unwrap();
                for dir_name in current_branch.iter().cloned() {
                    dirs.get_mut(&dir_name).unwrap().size += size;
                }
            },
            _ => println!("unreachable"),
        }
    }

    dirs.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_marker_index() {
        let command_results = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        let command_results: Vec<String> = command_results.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(sub_10000_dir_sizes(&command_results), 95437);
    }
}
