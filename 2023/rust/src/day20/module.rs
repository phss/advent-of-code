use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Broadcaster {
    label: String,
    destinations: Vec<String>,
}

impl FromStr for Broadcaster {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, destinations) = parse(s, "");
        Ok(Broadcaster {
            label,
            destinations,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FlipFlop {
    label: String,
    destinations: Vec<String>,
    on: bool,
}

impl FromStr for FlipFlop {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, destinations) = parse(s, "%");
        Ok(FlipFlop {
            label,
            destinations,
            on: false,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Conjunction {
    label: String,
    destinations: Vec<String>,
    input_pulses: HashMap<String, bool>,
}

impl FromStr for Conjunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, destinations) = parse(s, "&");
        Ok(Conjunction {
            label,
            destinations,
            input_pulses: HashMap::new(),
        })
    }
}

fn parse(s: &str, prefix: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = s.split(" -> ").collect();
    let label = parts[0].strip_prefix(prefix).unwrap().to_string();
    let destinations = parts[1].split(',').map(|s| s.trim().to_string()).collect();
    (label, destinations)
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
                    label: "broadcaster".to_string(),
                    destinations: vec!["a".to_string(), "b".to_string(), "c".to_string()]
                }
            );
        }
    }

    mod flip_flop {
        use super::*;

        #[test]
        fn create_from_str() {
            let flip_flop = FlipFlop::from_str("%a -> b, c").unwrap();

            assert_eq!(
                flip_flop,
                FlipFlop {
                    label: "a".to_string(),
                    on: false,
                    destinations: vec!["b".to_string(), "c".to_string()]
                }
            );
        }
    }

    mod conjunction {
        use std::collections::HashMap;

        use super::*;

        #[test]
        fn create_from_str() {
            let conjunction = Conjunction::from_str("&a -> b, c").unwrap();

            assert_eq!(
                conjunction,
                Conjunction {
                    label: "a".to_string(),
                    input_pulses: HashMap::new(),
                    destinations: vec!["b".to_string(), "c".to_string()]
                }
            );
        }
    }
}
