use crate::parser;

pub fn part1() -> usize {
    0
}

pub fn part2() -> usize {
    0
}

fn sum_hash(seq: &Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let init_seq: Vec<String> = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            .split(",")
            .map(|s| s.to_string())
            .collect();

        let result = sum_hash(&init_seq);

        assert_eq!(result, 1320);
    }

    #[test]
    fn sample_input_part_2() {}
}
