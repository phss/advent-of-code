use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

use crate::parser;

struct Connection {
    a: String,
    b: String,
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid connection format: {}", s));
        }
        Ok(Connection {
            a: parts[0].to_string(),
            b: parts[1].to_string(),
        })
    }
}

pub fn part1() -> u32 {
    let connections: Vec<Connection> = parser::read("data/day23.txt").unwrap();
    count_groups_with_t(&connections) as u32
}

pub fn part2() -> u32 {
    let connections: Vec<Connection> = parser::read("data/day23.txt").unwrap();
    let result = lan_party_password(&connections);
    println!("Result {result}");
    0
}

fn count_groups_with_t(connections: &Vec<Connection>) -> usize {
    let graph = graph_from(connections);
    let mut t_sets = HashSet::new();
    for (computer, connections) in graph.clone() {
        if !computer.starts_with("t") {
            continue;
        }

        for conn in connections.into_iter().combinations(2) {
            let a = conn.first().unwrap();
            let b = conn.last().unwrap();

            if graph.get(a).unwrap().contains(b) {
                let key: String = vec![computer.clone(), a.clone(), b.clone()]
                    .iter()
                    .sorted()
                    .join("-");
                t_sets.insert(key);
            }
        }
    }

    t_sets.len()
}

fn lan_party_password(connections: &Vec<Connection>) -> String {
    let graph = graph_from(connections);

    let all_sets = graph.keys().map(|start| {
        let mut set = HashSet::from([start.clone()]);

        for (node, connections) in graph.clone() {
            if connections.is_superset(&set) {
                set.insert(node);
            }
        }

        set
    });

    all_sets
        .max_by_key(|a| a.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

fn graph_from(connections: &Vec<Connection>) -> HashMap<String, HashSet<String>> {
    let mut conn_map = HashMap::new();
    for conn in connections {
        conn_map
            .entry(conn.a.clone())
            .or_insert(HashSet::new())
            .insert(conn.b.clone());
        conn_map
            .entry(conn.b.clone())
            .or_insert(HashSet::new())
            .insert(conn.a.clone());
    }
    conn_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka",
            "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka",
            "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de",
            "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn",
        ];
        let connections: Vec<Connection> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = count_groups_with_t(&connections);

        assert_eq!(result, 7);
    }

    #[test]
    fn sample_input_part_2() {
        let lines = vec![
            "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka",
            "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka",
            "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de",
            "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn",
        ];
        let connections: Vec<Connection> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = lan_party_password(&connections);

        assert_eq!(result, "co,de,ka,ta");
    }
}
