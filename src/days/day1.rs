use itertools::Itertools;

use crate::days;

pub struct Day {}

impl days::Day for Day {
    type Input = String;

    fn get_num(&self) -> u8 {
        1
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> String {
        input
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&mut self, input: &Self::Input) -> String {
        input
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
            .to_string()
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.clone()
    }
}
