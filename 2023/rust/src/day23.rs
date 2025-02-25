use std::collections::{HashMap, HashSet, VecDeque};

use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day23.txt").unwrap();
    let map = parse(&lines);
    longest_hike(&map, HashSet::new(), (1, 0), (139, 140))
}

pub fn part2() -> usize {
    let lines: Vec<String> = parser::read("data/day23.txt").unwrap();
    let map = parse(&lines);
    let (edges, weights) = to_graph(&map, (1, 0), (21, 22));
    longest_hike_no_slopes(&edges, &weights, Vec::new(), (1, 0), (139, 140), 0)
}

fn longest_hike(
    map: &Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    from: (usize, usize),
    to: (usize, usize),
) -> usize {
    if from == to {
        return visited.len();
    }

    let mut new_visited = visited.clone();
    new_visited.insert(from);

    let (x, y) = from;
    let mut max_steps = 0;

    let directions = [(1, 0, '>'), (-1, 0, '<'), (0, 1, 'v'), (0, -1, '^')];
    for (dir_x, dir_y, dir_char) in directions {
        let next_x = x.checked_add_signed(dir_x).unwrap_or(0);
        let next_y = y.checked_add_signed(dir_y).unwrap_or(0);
        let next_position = (next_x, next_y);
        let next_char = map[next_y][next_x];

        if new_visited.contains(&next_position) || (next_char != '.' && next_char != dir_char) {
            continue;
        }

        max_steps = max_steps.max(longest_hike(map, new_visited.clone(), next_position, to));
    }

    max_steps
}

type Node = (usize, usize);

fn to_graph(
    map: &Vec<Vec<char>>,
    from: (usize, usize),
    to: (usize, usize),
) -> (HashMap<Node, HashSet<Node>>, HashMap<(Node, Node), usize>) {
    let node_symbols = ['>', '<', 'v', '^'];
    let width = map[0].len();
    let height = map.len();

    let mut nodes = HashSet::new();
    nodes.insert(from);
    nodes.insert(to);
    for x in 0..width {
        for y in 0..height {
            if node_symbols.contains(&map[y][x]) {
                nodes.insert((x, y));
            }
        }
    }

    let mut edges = HashMap::new();
    let mut weights = HashMap::new();

    for node in nodes {
        let mut visited = HashSet::new();
        visited.insert(node);
        let mut search_heap = VecDeque::new();
        search_heap.push_back((node, 0));

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        while let Some(((x, y), distance)) = search_heap.pop_front() {
            for (dir_x, dir_y) in directions {
                let next_x = x.checked_add_signed(dir_x).unwrap_or(0).min(width - 1);
                let next_y = y.checked_add_signed(dir_y).unwrap_or(0).min(height - 1);
                let next_position = (next_x, next_y);
                let next_char = map[next_y][next_x];

                if visited.contains(&next_position) || next_char == '#' {
                    continue;
                }
                visited.insert(next_position);

                if next_char != '.' || next_position == from || next_position == to {
                    edges
                        .entry(node)
                        .or_insert(HashSet::new())
                        .insert(next_position);
                    edges
                        .entry(next_position)
                        .or_insert(HashSet::new())
                        .insert(node);
                    weights.insert((node, next_position), distance + 1);
                    weights.insert((next_position, node), distance + 1);
                } else {
                    search_heap.push_back((next_position, distance + 1));
                }
            }
        }
    }

    (edges, weights)
}

fn longest_hike_no_slopes(
    edges: &HashMap<Node, HashSet<Node>>,
    weights: &HashMap<(Node, Node), usize>,
    visited: Vec<(usize, usize)>,
    from: (usize, usize),
    to: (usize, usize),
    current_distance: usize,
) -> usize {
    if from == to {
        println!("{:?} -> {:?}", current_distance, visited);
        return current_distance;
    }

    let mut new_visited = visited.clone();
    new_visited.push(from);

    let mut max_steps = 0;
    for next_node in edges.get(&from).unwrap() {
        if new_visited.contains(next_node) {
            continue;
        }

        let next_max_steps = longest_hike_no_slopes(
            edges,
            weights,
            new_visited.clone(),
            next_node.clone(),
            to,
            current_distance + weights.get(&(from, next_node.clone())).unwrap(),
        );

        max_steps = max_steps.max(next_max_steps);
    }

    max_steps
}

fn parse(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let result = longest_hike(&map, HashSet::new(), (1, 0), (21, 22));

        assert_eq!(result, 94);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "#.#####################",
            "#.......#########...###",
            "#######.#########.#.###",
            "###.....#.>.>.###.#.###",
            "###v#####.#v#.###.#.###",
            "###.>...#.#.#.....#...#",
            "###v###.#.#.#########.#",
            "###...#.#.#.......#...#",
            "#####.#.#.#######.#.###",
            "#.....#.#.#.......#...#",
            "#.#####.#.#.#########v#",
            "#.#...#...#...###...>.#",
            "#.#.#v#######v###.###v#",
            "#...#.>.#...>.>.#.###.#",
            "#####v#.#.###v#.#.###.#",
            "#.....#...#...#.#.#...#",
            "#.#########.###.#.#.###",
            "#...###...#...#...#.###",
            "###.###.#.###v#####v###",
            "#...#...#.#.>.>.#.>.###",
            "#.###.###.#.###.#.#v###",
            "#.....###...###...#...#",
            "#####################.#",
        ];
        let lines: Vec<String> = lines.into_iter().map(|s| s.parse().unwrap()).collect();
        let map = parse(&lines);

        let (edges, weights) = to_graph(&map, (1, 0), (21, 22));
        // for (node, conns) in edges.iter() {
        //     println!("{:?} -> {:?}", node, conns);
        // }
        // println!("");

        // for (node, conns) in &weights {
        //     println!("{:?} -> {:?}", node, conns);
        // }
        // println!("");

        let result = longest_hike_no_slopes(&edges, &weights, Vec::new(), (1, 0), (21, 22), 0);

        assert_eq!(result, 154);
    }
}
