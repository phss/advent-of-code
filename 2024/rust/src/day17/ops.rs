pub fn mod8(value: usize) -> usize {
    value.rem_euclid(8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod8() {
        assert_eq!(mod8(3), 3);
        assert_eq!(mod8(9), 1);
        assert_eq!(mod8(231), 7);
    }
}
