use std::collections::HashSet;

use super::region::Region;

pub(crate) type Position = (usize, usize);

pub struct Map {
    pub raw: Vec<String>,
}

impl Map {
    pub fn iter(&self) -> MapIterator {
        MapIterator {
            map: self,
            x_index: 0,
            y_index: 0,
        }
    }

    pub fn get_region(&self, start: Position) -> Region {
        let mut nodes = HashSet::new();
        let mut to_visit = vec![start];
        let name = self.raw[start.1].chars().nth(start.0).unwrap_or(' ');

        while let Some(position @ (x, y)) = to_visit.pop() {
            if nodes.contains(&position) {
                continue;
            }
            nodes.insert(position);

            let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dir_x, dir_y) in directions.iter() {
                let new_x = x.checked_add_signed(*dir_x);
                let new_y = y.checked_add_signed(*dir_y);
                let new_position = new_x.zip(new_y);

                let new_region = new_position.and_then(|(new_x, new_y)| {
                    self.raw
                        .get(new_y)
                        .unwrap_or(&String::new())
                        .chars()
                        .nth(new_x)
                });

                if new_region == Some(name) {
                    to_visit.push(new_position.unwrap());
                }
            }
        }

        Region { name, nodes }
    }
}

pub struct MapIterator<'a> {
    map: &'a Map,
    x_index: usize,
    y_index: usize,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (char, Position);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .map
            .raw
            .get(self.y_index)
            .unwrap_or(&String::new())
            .chars()
            .nth(self.x_index)
            .map(|region_name| (region_name, (self.x_index, self.y_index)));

        self.x_index += 1;
        if self.x_index == self.map.raw.first().map(String::len).unwrap_or(0) {
            self.x_index = 0;
            self.y_index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_iterator() {
        let map = Map {
            raw: vec!["abc".to_string(), "def".to_string(), "ghi".to_string()],
        };

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some(('a', (0, 0))));
        assert_eq!(iter.next(), Some(('b', (1, 0))));
        assert_eq!(iter.next(), Some(('c', (2, 0))));
        assert_eq!(iter.next(), Some(('d', (0, 1))));
        assert_eq!(iter.next(), Some(('e', (1, 1))));
        assert_eq!(iter.next(), Some(('f', (2, 1))));
        assert_eq!(iter.next(), Some(('g', (0, 2))));
        assert_eq!(iter.next(), Some(('h', (1, 2))));
        assert_eq!(iter.next(), Some(('i', (2, 2))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_map_iterator() {
        let map = Map { raw: vec![] };

        let mut iter = map.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_row_map_iterator() {
        let map = Map {
            raw: vec!["abc".to_string()],
        };

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some(('a', (0, 0))));
        assert_eq!(iter.next(), Some(('b', (1, 0))));
        assert_eq!(iter.next(), Some(('c', (2, 0))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_single_column_map_iterator() {
        let map = Map {
            raw: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some(('a', (0, 0))));
        assert_eq!(iter.next(), Some(('b', (0, 1))));
        assert_eq!(iter.next(), Some(('c', (0, 2))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_get_region() {
        let map = Map {
            raw: vec!["aaa".to_string(), "aba".to_string(), "aaa".to_string()],
        };

        let region = map.get_region((0, 0));
        let expected_nodes: HashSet<Position> = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (0, 1),
            (2, 1),
            (0, 2),
            (1, 2),
            (2, 2),
        ]
        .into_iter()
        .collect();
        assert_eq!(region.nodes, expected_nodes);

        let region = map.get_region((1, 1));
        let expected_nodes: HashSet<Position> = vec![(1, 1)].into_iter().collect();
        assert_eq!(region.nodes, expected_nodes);
    }
}
