use std::collections::HashMap;

use crate::parser;

#[derive(Clone, Debug)]
struct DirSizes {
    size: u32,
}

pub fn part1() -> u32 {
    let command_results: Vec<String> = parser::read("data/day7.txt").unwrap();
    sub_10000_dir_sizes(&command_results)
}

pub fn part2() -> u32 {
    let command_results: Vec<String> = parser::read("data/day7.txt").unwrap();
    smallest_delete(&command_results)
}

fn sub_10000_dir_sizes(command_results: &Vec<String>) -> u32 {
    let dirs: Vec<DirSizes> = parse_to_dir_sizes(command_results);
    println!("{:?}", dirs);
    dirs.into_iter()
        .filter(|dir| dir.size <= 100000)
        .map(|dir| dir.size)
        .sum()
}

fn smallest_delete(command_results: &Vec<String>) -> u32 {
    let dirs: Vec<DirSizes> = parse_to_dir_sizes(command_results);
    let sizes: Vec<u32> = dirs.into_iter().map(|dir| dir.size).collect();
    let root_size = sizes.iter().max().unwrap();
    let remaining = 70000000 - root_size;
    let to_delete = 30000000 - remaining;

    sizes
        .into_iter()
        .filter(|&size| size > to_delete)
        .min()
        .unwrap()
}

fn parse_to_dir_sizes(command_results: &Vec<String>) -> Vec<DirSizes> {
    let mut dirs: HashMap<String, DirSizes> = HashMap::new();
    let mut current_branch: Vec<String> = vec![];

    for line in command_results {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        match parts[..] {
            ["$", "ls"] | ["dir", _] => {}
            ["$", "cd", ".."] => {
                current_branch.pop();
            }
            ["$", "cd", dir] => {
                let path = if let Some(path) = current_branch.last() {
                    path
                } else {
                    ""
                };
                let dir_path = format!("{}/{}", path, dir);
                current_branch.push(format!("{}/{}", path, dir));
                dirs.insert(dir_path, DirSizes { size: 0 });
            }
            [size, _] => {
                let size: u32 = size.parse().unwrap();
                for dir_path in current_branch.iter().cloned() {
                    dirs.get_mut(&dir_path).unwrap().size += size;
                }
            }
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
        let command_results: Vec<String> =
            command_results.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(sub_10000_dir_sizes(&command_results), 95437);
    }

    #[test]
    fn sample_input_smallest_delete() {
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
        let command_results: Vec<String> =
            command_results.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(smallest_delete(&command_results), 24933642);
    }
}
