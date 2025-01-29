use crate::parser;

pub fn part1() -> usize {
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    let init_seq = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();
    sum_hash(&init_seq)
}

pub fn part2() -> usize {
    0
}

fn sum_hash(seq: &Vec<String>) -> usize {
    seq.iter().map(hash).sum()
}

fn hash(str: &String) -> usize {
    Vec::from(str.clone()).iter().fold(0, |acc, c| {
        let mut new_acc = acc + *c as usize;
        new_acc *= 17;
        new_acc %= 256;
        new_acc
    })
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
