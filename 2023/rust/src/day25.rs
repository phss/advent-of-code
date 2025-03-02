mod graph;

use rand::prelude::*;
use std::str::FromStr;

use graph::Graph;

use crate::parser;

#[derive(Debug)]
struct Connection {
    component: String,
    others: Vec<String>,
}

impl Connection {
    fn to_pairs(&self) -> Vec<(&str, &str)> {
        self.others
            .iter()
            .map(|other| (self.component.as_str(), other.as_str()))
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
    let pairs: Vec<(&str, &str)> = connections.iter().flat_map(Connection::to_pairs).collect();
    let graph = Graph::from_pairs(pairs);

    loop {
        let mut candidate = graph.clone();
        krager_min_cut(&mut candidate);

        if candidate.weights.values().all(|weight| *weight == 3) {
            return candidate
                .nodes
                .values()
                .fold(1, |acc, node| acc * node.count);
        }
    }
}

fn krager_min_cut(graph: &mut Graph) {
    let mut rng = rand::rng();

    while graph.nodes.len() > 2 {
        let a = graph.nodes.keys().choose(&mut rng).unwrap();
        let b = graph.edges.get(a).unwrap().iter().choose(&mut rng).unwrap();
        graph.contract_nodes(a, b);
    }
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
