pub fn bst(value: usize) -> usize {
    value.rem_euclid(8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        assert_eq!(bst(3), 3);
        assert_eq!(bst(9), 1);
        assert_eq!(bst(231), 7);
    }
}
