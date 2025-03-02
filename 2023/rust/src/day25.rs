use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::parser;
use rand::seq::IteratorRandom;

#[derive(Debug)]
struct Connection {
    component: String,
    others: Vec<String>,
}

impl Connection {
    fn to_pairs(&self) -> Vec<(String, String)> {
        self.others
            .iter()
            .map(|other| (self.component.clone(), other.clone()))
            .collect()
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();

        let component = parts[0].to_string();
        let others = parts[1].split_whitespace().map(|s| s.to_string()).collect();

        Ok(Connection { component, others })
    }
}

pub fn part1() -> usize {
    let connections: Vec<Connection> = parser::read("data/day25.txt").unwrap();
    disconnect_and_sum(&connections)
}

pub fn part2() -> usize {
    0
}

fn disconnect_and_sum(connections: &Vec<Connection>) -> usize {
    let pairs: Vec<(String, String)> = connections.iter().flat_map(Connection::to_pairs).collect();
    let edges = to_graph(pairs);

    let blah = karger_min_cut(edges);
    println!("{:?}", blah);

    0
}

fn karger_min_cut(
    init: HashMap<String, HashSet<(String, usize)>>,
) -> HashMap<String, HashSet<(String, usize)>> {
    let mut edges = init.clone();
    let mut rng = rand::rng();

    while edges.len() > 2 {
        let a = edges.keys().choose(&mut rng).unwrap().clone();
        let mut a_edges = edges.get_mut(&a).unwrap();
        let b = a_edges.iter().choose(&mut rng).unwrap().0.clone();
        let b_edges = edges.get(&b).unwrap();
    }

    edges
}

fn to_graph(pairs: Vec<(String, String)>) -> HashMap<String, HashSet<(String, usize)>> {
    let mut edges = HashMap::new();

    for (a, b) in pairs {
        edges
            .entry(a.clone())
            .or_insert(HashSet::new())
            .insert((b.clone(), 1));
        edges
            .entry(b.clone())
            .or_insert(HashSet::new())
            .insert((a.clone(), 1));
    }

    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let lines = vec![
            "jqt: rhn xhk nvd",
            "rsh: frs pzl lsr",
            "xhk: hfx",
            "cmg: qnr nvd lhk bvb",
            "rhn: xhk bvb hfx",
            "bvb: xhk hfx",
            "pzl: lsr hfx nvd",
            "qnr: nvd",
            "ntq: jqt hfx bvb xhk",
            "nvd: lhk",
            "lsr: lhk",
            "rzs: qnr cmg lsr rsh",
            "frs: qnr lhk lsr",
        ];
        let connections: Vec<Connection> = lines.into_iter().map(|s| s.parse().unwrap()).collect();

        let result = disconnect_and_sum(&connections);

        assert_eq!(result, 54);
    }

    #[test]
    fn sample_input_part_2() {}
}
