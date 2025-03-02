use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

use crate::parser;

type Point = (i32, i32);

#[derive(Debug, Clone, PartialEq)]
struct Sensor {
    at: Point,
    beacon: Point,
}

impl Sensor {
    fn distance(self: &Self) -> i32 {
        let x_distance = (self.at.0 - self.beacon.0).abs();
        let y_distance = (self.at.1 - self.beacon.1).abs();
        x_distance + y_distance
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ParseSensorError;

impl FromStr for Sensor {
    type Err = ParseSensorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted_string = s.split(": ");

        let mut sensor_parts = splitted_string.next().unwrap()[10..].split(", ");
        let sensor_x = sensor_parts.next().unwrap()[2..].parse().unwrap();
        let sensor_y = sensor_parts.next().unwrap()[2..].parse().unwrap();

        let mut beacon_parts = splitted_string.next().unwrap()[21..].split(", ");
        let beacon_x = beacon_parts.next().unwrap()[2..].parse().unwrap();
        let beacon_y = beacon_parts.next().unwrap()[2..].parse().unwrap();

        Ok(Self {
            at: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        })
    }
}

pub fn part1() -> usize {
    let sensors: Vec<Sensor> = parser::read("data/day15.txt").unwrap();
    no_beacon_positions(&sensors, 2000000)
}

pub fn part2() -> usize {
    let sensors: Vec<Sensor> = parser::read("data/day15.txt").unwrap();
    tunning_frequency(&sensors, 4000000)
}

fn no_beacon_positions(sensors: &Vec<Sensor>, at_y: i32) -> usize {
    unique_ranges_at_y(sensors, at_y)
        .into_iter()
        .map(|range| range.count())
        .sum::<usize>() as usize
        - 1
}

fn unique_ranges_at_y(sensors: &Vec<Sensor>, at_y: i32) -> Vec<RangeInclusive<i32>> {
    let all_ranges_at_y: Vec<RangeInclusive<i32>> = sensors
        .iter()
        .filter(|sensor| {
            (sensor.at.1 - sensor.distance()..=sensor.at.1 + sensor.distance()).contains(&at_y)
        })
        .map(|sensor| {
            let distance_to_y = (sensor.at.1 - at_y).abs();
            let width_in_y = (sensor.distance() - distance_to_y).abs();
            sensor.at.0 - width_in_y..=sensor.at.0 + width_in_y
        })
        .sorted_by_key(|sensor| *sensor.start())
        .collect();

    all_ranges_at_y[1..]
        .iter()
        .fold(vec![all_ranges_at_y[0].clone()], |mut acc, range| {
            let last_range = acc.last().unwrap();
            if last_range.contains(range.start()) {
                let last_range = *last_range.start()..=*range.end().max(last_range.end());
                let last_index = acc.len() - 1;
                acc[last_index] = last_range;
                return acc;
            } else {
                acc.push(range.clone());
            }
            acc
        })
}

fn tunning_frequency(sensors: &Vec<Sensor>, max: i32) -> usize {
    for y in 0..=max {
        let ranges = unique_ranges_at_y(sensors, y);
        if ranges.len() != 1 {
            println!("{} {:?}", y, ranges);
            // sadly doing the math by hand after getting the line
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let line = String::from("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");
        assert_eq!(
            Sensor::from_str(&line),
            Ok(Sensor {
                at: (2, 18),
                beacon: (-2, 15)
            })
        );
    }

    #[test]
    fn sample_input_part_1() {
        let sensors: Vec<Sensor> = vec![
            String::from("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            String::from("Sensor at x=9, y=16: closest beacon is at x=10, y=16"),
            String::from("Sensor at x=13, y=2: closest beacon is at x=15, y=3"),
            String::from("Sensor at x=12, y=14: closest beacon is at x=10, y=16"),
            String::from("Sensor at x=10, y=20: closest beacon is at x=10, y=16"),
            String::from("Sensor at x=14, y=17: closest beacon is at x=10, y=16"),
            String::from("Sensor at x=8, y=7: closest beacon is at x=2, y=10"),
            String::from("Sensor at x=2, y=0: closest beacon is at x=2, y=10"),
            String::from("Sensor at x=0, y=11: closest beacon is at x=2, y=10"),
            String::from("Sensor at x=20, y=14: closest beacon is at x=25, y=17"),
            String::from("Sensor at x=17, y=20: closest beacon is at x=21, y=22"),
            String::from("Sensor at x=16, y=7: closest beacon is at x=15, y=3"),
            String::from("Sensor at x=14, y=3: closest beacon is at x=15, y=3"),
            String::from("Sensor at x=20, y=1: closest beacon is at x=15, y=3"),
        ]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

        assert_eq!(no_beacon_positions(&sensors, 10), 26);
    }

    #[test]
    fn sample_input_part_2() {}
}
