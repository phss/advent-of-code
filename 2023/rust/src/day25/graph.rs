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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_from_pairs() {
        let pairs = vec![("a", "b"), ("c", "b"), ("a", "c")];

        let actual = Graph::from_pairs(pairs);

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

        assert_eq!(actual, expected);
    }
}
