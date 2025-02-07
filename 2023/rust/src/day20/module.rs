use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Broadcaster {
    destinations: Vec<String>,
}

impl FromStr for Broadcaster {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("->").collect();
        let values = parts[1].split(',').map(|s| s.trim().to_string()).collect();
        Ok(Broadcaster {
            destinations: values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod broadcaster {
        use super::*;

        #[test]
        fn create_from_str() {
            let broadcaster = Broadcaster::from_str("broadcaster -> a, b, c").unwrap();

            assert_eq!(
                broadcaster,
                Broadcaster {
                    destinations: vec!["a".to_string(), "b".to_string(), "c".to_string()]
                }
            );
        }
    }
}
