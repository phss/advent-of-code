use itertools::Itertools;

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
    let lines: Vec<String> = parser::read("data/day15.txt").unwrap();
    let init_seq = lines
        .first()
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();
    focusing_power(&init_seq)
}

fn sum_hash(seq: &Vec<String>) -> usize {
    seq.iter().map(hash).sum()
}

fn focusing_power(seq: &Vec<String>) -> usize {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

    for s in seq {
        let (label, operation, focal_length) = parse(s);
        let box_id = hash(&label);
        let label_pos = boxes[box_id].iter().find_position(|(l, _)| *l == label);

        match operation {
            '=' => {
                if let Some((index, _)) = label_pos {
                    boxes[box_id][index].1 = focal_length.unwrap();
                } else {
                    boxes[box_id].push((label.clone(), focal_length.unwrap()));
                }
            }
            '-' => {
                if let Some((index, _)) = label_pos {
                    boxes[box_id].remove(index);
                }
            }
            _ => panic!("unreachable"),
        }
    }

    let mut sum = 0;
    for (box_id, bs) in boxes.iter().enumerate() {
        for (slot_id, (_, focal_length)) in bs.iter().enumerate() {
            sum += (box_id + 1) * (slot_id + 1) * focal_length;
        }
    }
    sum
}

fn parse(s: &String) -> (String, char, Option<usize>) {
    let operation = if s.contains("=") { '=' } else { '-' };
    let mut parts = s.split(operation);
    let label = parts.next().unwrap().to_string();
    let focal_length: Option<usize> = if operation == '=' {
        parts.next().map(|c| c.to_string().parse().unwrap())
    } else {
        None
    };

    (label, operation, focal_length)
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
    fn sample_input_part_2() {
        let init_seq: Vec<String> = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            .split(",")
            .map(|s| s.to_string())
            .collect();

        let result = focusing_power(&init_seq);

        assert_eq!(result, 145);
    }
}
