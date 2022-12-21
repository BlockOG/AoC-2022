use itertools::Itertools;

use crate::days;

pub struct Day {
    day_num: u8,
}

impl days::Day for Day {
    type Input = String;

    fn get_num(&self) -> u8 {
        self.day_num
    }

    fn new(day_num: u8) -> Self {
        Self {
            day_num
        }
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        (
            input
                .split("\n\n")
                .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
                .max()
                .unwrap()
                .to_string(),
            true,
        )
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        (
            input
                .split("\n\n")
                .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).sum::<i32>())
                .sorted()
                .rev()
                .take(3)
                .sum::<i32>()
                .to_string(),
            true,
        )
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.clone()
    }
}
