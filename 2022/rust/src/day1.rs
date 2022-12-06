use crate::parser;

pub fn part1() -> u32 {
    let lines: Vec<String> = parser::read("data/day1.txt").unwrap();
    let food_calories = to_food_calories_per_elf(&lines);
    max_carried_calories(&food_calories)
}

fn to_food_calories_per_elf(lines: &Vec<String>) -> Vec<Vec<u32>> {
    lines
        .split(|line| line.is_empty())
        .map(|lines| lines.iter().map(|s| s.parse().unwrap()).collect() )
        .collect()
}

fn max_carried_calories(food_calories: &Vec<Vec<u32>>) -> u32 {
    food_calories
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increases_in_sample_input() {
        let food_calories = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000]
        ];
        assert_eq!(max_carried_calories(&food_calories), 24000);
    }

}
