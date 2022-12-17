use std::{collections::HashMap, slice::Iter};

use crate::parser;

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum DirTree {
    Dir {
        name: String,
        children: Vec<DirTree>,
    },
    File {
        name: String,
        size: u32,
    },
}

#[allow(dead_code)]
impl DirTree {
    pub fn size(self: Self) -> u32 {
        match self {
            DirTree::File { name: _, size } => size,
            DirTree::Dir { name: _, children } => children.into_iter().map(DirTree::size).sum(),
        }
    }

    pub fn parse(command_results: Vec<String>) -> DirTree {
        let mut command_results = command_results.iter();
        command_results.next(); // skip the first $ cd /
        Self::parse_in_dir("/".to_string(), &mut command_results)
    }

    fn parse_in_dir(name: String, command_results: &mut Iter<String>) -> DirTree {
        let mut children: Vec<DirTree> = vec![];

        while let Some(line) = command_results.next() {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            match parts[..] {
                ["$", "ls"] | ["dir", _] => {}
                ["$", "cd", ".."] => {
                    break;
                }
                ["$", "cd", name] => {
                    children.push(Self::parse_in_dir(name.to_string(), command_results));
                }
                [size, name] => children.push(DirTree::File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                }),
                _ => println!("unreachable"),
            }
        }

        DirTree::Dir { name, children }
    }
}

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
    fn sample_input_parsing() {
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

        assert_eq!(
            DirTree::parse(command_results),
            DirTree::Dir {
                name: "/".to_string(),
                children: vec![
                    DirTree::File {
                        name: "b.txt".to_string(),
                        size: 14848514
                    },
                    DirTree::File {
                        name: "c.dat".to_string(),
                        size: 8504156
                    },
                    DirTree::Dir {
                        name: "a".to_string(),
                        children: vec![
                            DirTree::File {
                                name: "f".to_string(),
                                size: 29116
                            },
                            DirTree::File {
                                name: "g".to_string(),
                                size: 2557
                            },
                            DirTree::File {
                                name: "h.lst".to_string(),
                                size: 62596
                            },
                            DirTree::Dir {
                                name: "e".to_string(),
                                children: vec![DirTree::File {
                                    name: "i".to_string(),
                                    size: 584
                                },]
                            },
                        ]
                    },
                    DirTree::Dir {
                        name: "d".to_string(),
                        children: vec![
                            DirTree::File {
                                name: "j".to_string(),
                                size: 4060174
                            },
                            DirTree::File {
                                name: "d.log".to_string(),
                                size: 8033020
                            },
                            DirTree::File {
                                name: "d.ext".to_string(),
                                size: 5626152
                            },
                            DirTree::File {
                                name: "k".to_string(),
                                size: 7214296
                            },
                        ]
                    },
                ]
            }
        );
    }

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
