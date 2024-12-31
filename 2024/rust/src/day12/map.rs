type Positon = (usize, usize);

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
}

pub struct MapIterator<'a> {
    map: &'a Map,
    x_index: usize,
    y_index: usize,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (char, Positon);

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
}
