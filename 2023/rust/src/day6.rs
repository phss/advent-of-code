pub fn part1() -> usize {
    let times = vec![34, 90, 89, 86];
    let distances = vec![204, 1713, 1210, 1780];
    mult_of_number_of_ways(&times, &distances)
}

pub fn part2() -> usize {
    let times = vec![34908986];
    let distances = vec![204171312101780];
    mult_of_number_of_ways(&times, &distances)
}

fn mult_of_number_of_ways(times: &Vec<usize>, distances: &Vec<usize>) -> usize {
    let mut mult = 1;

    times
        .iter()
        .zip(distances.iter())
        .for_each(|(time, distance_goal)| mult *= num_of_ways_to_win(*time, *distance_goal));

    mult
}

fn num_of_ways_to_win(time: usize, distance_goal: usize) -> usize {
    let mut ways = 0;

    for pressed in 1..=time {
        let remaining_time = time - pressed;
        let distance = remaining_time * pressed;
        if distance > distance_goal {
            ways += 1;
        }
    }

    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_1() {
        let times = vec![7, 15, 30];
        let distances = vec![9, 40, 200];

        let result = mult_of_number_of_ways(&times, &distances);

        assert_eq!(result, 288);
    }

    #[test]
    fn sample_input_part_2() {
        let times = vec![71530];
        let distances = vec![940200];

        let result = mult_of_number_of_ways(&times, &distances);

        assert_eq!(result, 71503);
    }
}
