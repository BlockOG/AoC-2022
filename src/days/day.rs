use crate::days;

pub struct Day {}

impl days::Day for Day {
    type Input = String;

    fn get_num(&self) -> u8 {
        0
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        todo!("Part 1 Day {}", self.get_num())
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        todo!("Part 2 Day {}", self.get_num())
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.clone()
    }
}
