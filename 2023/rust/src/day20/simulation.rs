use crate::day20::module::Broadcaster;

pub struct Simulation {
    pub broadcaster: Broadcaster,
}

impl Simulation {
    pub fn parse(lines: &Vec<String>) -> Self {
        let broadcaster = lines
            .iter()
            .find(|line| line.starts_with("broadcaster"))
            .map(|line| line.parse().unwrap())
            .unwrap();

        Simulation { broadcaster }
    }
}
