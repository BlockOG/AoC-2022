use crate::days;

pub struct Day {}

impl days::Day for Day {
    type Input = String;

    fn get_num(&self) -> u8 {
        0
    }

    fn part1(&self, input: &Self::Input) -> String {
        todo!("Part 1 Day {}", self.get_num())
    }

    fn part2(&self, input: &Self::Input) -> String {
        todo!("Part 2 Day {}", self.get_num())
    }

    fn parse_input(&self, input: &String) -> Self::Input {
        input.clone()
    }
}
