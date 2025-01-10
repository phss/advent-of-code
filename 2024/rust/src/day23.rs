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
    0
}

fn count_groups_with_t(connections: &Vec<Connection>) -> usize {
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

    let mut t_sets = HashSet::new();
    for (computer, connections) in conn_map.clone() {
        if !computer.starts_with("t") {
            continue;
        }

        for conn in connections.into_iter().combinations(2) {
            let a = conn.first().unwrap();
            let b = conn.last().unwrap();

            if conn_map.get(a).unwrap().contains(b) {
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
    fn sample_input_part_2() {}
}
