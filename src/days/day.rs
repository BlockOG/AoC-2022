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

    fn part1(&mut self, _input: &Self::Input) -> (String, bool) {
        todo!("Part 1 Day {}", self.get_num())
    }

    fn part2(&mut self, _input: &Self::Input) -> (String, bool) {
        todo!("Part 2 Day {}", self.get_num())
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input.clone()
    }
}
