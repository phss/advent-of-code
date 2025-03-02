use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<'a> {
    label: &'a str,
    count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    edges: HashMap<&'a str, HashSet<&'a str>>,
    weights: HashMap<(&'a str, &'a str), usize>,
}

impl<'a> Graph<'a> {
    pub fn from_pairs(pairs: Vec<(&'a str, &'a str)>) -> Self {
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();
        let mut weights = HashMap::new();

        for (a, b) in pairs {
            nodes.entry(a).or_insert(Node { label: a, count: 1 });
            nodes.entry(b).or_insert(Node { label: b, count: 1 });

            edges.entry(a).or_insert(HashSet::new()).insert(b);
            edges.entry(b).or_insert(HashSet::new()).insert(a);

            weights.entry((a, b)).or_insert(1);
            weights.entry((b, a)).or_insert(1);
        }

        Graph {
            nodes,
            edges,
            weights,
        }
    }

    pub fn contract_nodes(&mut self, a: &'a str, b: &'a str) {
        let b_node_count = self.nodes.get(b).unwrap().count;
        self.nodes
            .entry(a)
            .and_modify(|node| node.count += b_node_count);
        self.nodes.remove(b);

        for c in self.edges.get(b).unwrap().clone() {
            self.edges.entry(c).and_modify(|node| {
                node.remove(b);
            });

            if c != a {
                self.edges.entry(a).and_modify(|node| {
                    node.insert(c);
                });
                self.edges.entry(c).and_modify(|node| {
                    node.insert(a);
                });

                let b_c_weight = self.weights.get(&(b, c)).unwrap().clone();
                *self.weights.entry((a, c)).or_insert(0) += b_c_weight;
                *self.weights.entry((c, a)).or_insert(0) += b_c_weight;

                self.weights.remove(&(b, c));
                self.weights.remove(&(c, b));
            }
        }
        self.edges.remove(b);
        self.weights.remove(&(a, b));
        self.weights.remove(&(b, a));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_pairs() {
        let pairs = vec![("a", "b"), ("c", "b"), ("a", "c")];

        let graph = Graph::from_pairs(pairs);

        #[rustfmt::skip]
        let expected = Graph {
            nodes: HashMap::from([
                ("a", Node { label: "a", count: 1 }),
                ("b", Node { label: "b", count: 1 }),
                ("c", Node { label: "c", count: 1 }),
            ]),
            edges: HashMap::from([
                ("a", HashSet::from(["b", "c"])),
                ("b", HashSet::from(["a", "c"])),
                ("c", HashSet::from(["a", "b"])),
            ]),
            weights: HashMap::from([
                (("a", "b"), 1),
                (("b", "a"), 1),
                (("c", "b"), 1),
                (("b", "c"), 1),
                (("a", "c"), 1),
                (("c", "a"), 1),
            ]),
        };

        assert_eq!(graph, expected);
    }

    #[test]
    fn contract_nodes() {
        let pairs = vec![("a", "b"), ("c", "b"), ("a", "c"), ("a", "d"), ("b", "e")];

        let mut graph = Graph::from_pairs(pairs);
        graph.contract_nodes("a", "b");

        #[rustfmt::skip]
        let expected = Graph {
            nodes: HashMap::from([
                ("a", Node { label: "a", count: 2 }),
                ("c", Node { label: "c", count: 1 }),
                ("d", Node { label: "d", count: 1 }),
                ("e", Node { label: "e", count: 1 }),
            ]),
            edges: HashMap::from([
                ("a", HashSet::from(["c", "d", "e"])),
                ("c", HashSet::from(["a"])),
                ("d", HashSet::from(["a"])),
                ("e", HashSet::from(["a"])),
            ]),
            weights: HashMap::from([
                (("a", "c"), 2),
                (("c", "a"), 2),
                (("a", "d"), 1),
                (("d", "a"), 1),
                (("a", "e"), 1),
                (("e", "a"), 1),
            ]),
        };

        assert_eq!(graph, expected);
    }
}
